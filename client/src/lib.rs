use common::models::{ClientState, GameCommand, GameState, ServerToClient};
use failure::Error;
use futures::{Future, Sink, Stream};
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio_tungstenite as tokio_ws;
use tokio_ws::tungstenite as ws;

pub mod ship;

/// `Handler` is provided as the trait that players can implement to interact
/// with the game server.
pub trait Handler {
    /// An opportunity, provided multiple times a second, to analyze the current
    /// state of the world and do a single action based on its state.
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand>;
}

fn log_err<E: Debug>(e: E) {
    eprintln!("{:?}", e)
}

fn build_game_loop<H, S, D>(
    sink: S,
    client_state: Arc<Mutex<ClientState>>,
    mut handler: H,
) -> impl Future<Item = (), Error = ()>
where
    H: Handler + Send + 'static,
    S: Sink<SinkItem = ws::Message, SinkError = D>,
    D: Debug,
{
    // Create a stream that produces at our desired interval
    tokio::timer::Interval::new_interval(Duration::from_millis(100))
        // Give the user a chance to take a turn
        .filter_map(move |_| {
            let client_state = client_state.lock().unwrap();
            handler.tick(&*client_state)
        })
        // Convert their command to a websocket message
        .map(move |command: GameCommand| {
            ws::Message::Text(serde_json::to_string(&command).unwrap())
        })
        // Satisfy the type gods.
        .map_err(log_err)
        // And send the message out.
        .forward(sink.sink_map_err(log_err))
        .map(|_| ()) // throw away leftovers from forward
}

fn build_state_updater<S, D>(
    stream: S,
    client_state: Arc<Mutex<ClientState>>,
) -> impl Future<Item = (), Error = ()>
where
    S: Stream<Item = ws::Message, Error = D>,
    D: Debug,
{
    stream
        // We only care about text websocket messages.
        .filter_map(|message| message.into_text().ok())
        // We especially only care about proper JSON messages.
        .filter_map(|message| serde_json::from_str(&message).ok())
        // Update the our game state to the most recent reported by the server.
        .for_each(move |server_to_client_msg| {
            match server_to_client_msg {
                ServerToClient::Id(player_id) => {
                    (*client_state).lock().unwrap().id = player_id;
                },
                ServerToClient::GameState(state) => {
                    (*client_state).lock().unwrap().game_state = state;
                },
            }

            Ok(())
        })
        .map_err(log_err)
}

/// Begin the client-side game loop, using the provided struct that implements `Handler`
/// to act on behalf of the player.
pub fn run<H>(key: &str, handler: H) -> Result<(), Error>
where
    H: Handler + Send + 'static,
{
    let url = url::Url::parse(&format!("ws://127.0.0.1:3000/socket?key={}", key))?;

    let client_state = Arc::new(Mutex::new(ClientState {
        id: 0,
        game_state: GameState::default(),
    }));

    let client = tokio_ws::connect_async(url)
        .and_then(move |(websocket, _)| {
            // Allow us to build two futures out of this connection - one for send, one for recv.
            let (sink, stream) = websocket.split();

            let game_loop = build_game_loop(sink, client_state.clone(), handler);
            let state_updater = build_state_updater(stream, client_state);

            // Return a future that will finish when either one of the two futures finish.
            state_updater.select(game_loop).then(|_| Ok(()))
        })
        .map_err(log_err);

    tokio::run(client);
    Ok(())
}

#[cfg(test)]
mod tests {}
