use common::models::{GameCommand, GameState};
use failure::Error;
use futures::{Future, Sink, Stream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_tungstenite as tokio_ws;
use tokio_ws::tungstenite as ws;

/// `Handler` is provided as the trait that players can implement to interact
/// with the game server.
pub trait Handler {
    /// An opportunity, provided multiple times a second, to analyze the current
    /// state of the world and do a single action based on its state.
    fn tick(&mut self, state: &GameState) -> GameCommand;
}

fn log_err<E>(e: E) where E: std::fmt::Debug {
    eprintln!("{:?}", e)
}

/// Begin the client-side game loop, using the provided struct that implements `Handler`
/// to act on behalf of the player.
pub fn run<H>(mut handler: H) -> Result<(), Error> where H: Handler + Send + 'static {
    let url = url::Url::parse("ws://127.0.0.1:3000/socket")?;

    let game_state = Arc::new(Mutex::new(GameState { counter: 0 }));
    let client = tokio_ws::connect_async(url).and_then(move |(websocket, _)| {

        // Allow us to build two futures out of this connection - one for send, one for recv.
        let (sink, stream) = websocket.split();

        // Create a stream that produces at our desired interval
        let command_sender = tokio::timer::Interval::new_interval(Duration::from_millis(100))
            // Give the user a chance to take a turn
            .map({
                let game_state = game_state.clone();
                move |_| {
                    let game_state = game_state.lock().unwrap();
                    handler.tick(&*game_state)
                }
            })
            // Convert their command to a websocket message
            .map(move |command| ws::Message::Text(serde_json::to_string(&command).unwrap()))
            // Satisfy the type gods.
            .map_err(log_err)
            // And send the message out.
            .forward(sink.sink_map_err(log_err))
            .map(|_| ()); // throw away leftovers from forward

        let state_updater = stream
            // We only care about text websocket messages.
            .filter_map(|message| message.into_text().ok())
            // We especially only care about proper JSON messages.
            .filter_map(|message| serde_json::from_str(&message).ok())
            // Update the our game state to the most recent reported by the server.
            .for_each(move |message| {
                *game_state.lock().unwrap() = message;
                Ok(())
            })
            .map_err(log_err);

        // Return a future that will finish when either one of the two futures finish.
        state_updater.select(command_sender)
            .then(|_| Ok(()))
    }).map_err(log_err);

    tokio::run(client);
    Ok(())
}

#[cfg(test)]
mod tests {
}
