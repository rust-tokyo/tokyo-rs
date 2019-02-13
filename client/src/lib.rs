use common::models::{GameCommand, GameState};
use failure::Error;
use futures::{Future, Sink, Stream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use spin_sleep::LoopHelper;
use tokio_tungstenite as tokio_ws;
use tokio_ws::tungstenite as ws;

pub trait Handler {
    fn tick(&mut self, state: &GameState) -> GameCommand;
}

pub fn run<H>(mut handler: H) -> Result<(), Error> where H: Handler + Send + 'static {
    let url = url::Url::parse("ws://127.0.0.1:3000/socket")?;

    let game_state = Arc::new(Mutex::new(GameState { counter: 0 }));
    let client = tokio_ws::connect_async(url).and_then(move |(websocket, _)| {
        println!("connected!");

        let (sink, stream) = websocket.split();

        let command_state = game_state.clone();
        let command_sender = tokio::timer::Interval::new_interval(Duration::from_millis(100))
            .map(move |_| handler.tick(&*command_state.lock().unwrap()))
            .map(move |command| ws::Message::Text(serde_json::to_string(&command).unwrap()))
            .map_err(|e| eprintln!("{:?}", e))
            .forward(sink.sink_map_err(|e| eprintln!("{:?}", e)))
            .map(|_| ()); // throw away leftovers from forward

        let updater_state = game_state.clone();
        let state_updater = stream
            .filter_map(|message| message.into_text().ok())
            .filter_map(|message| serde_json::from_str(&message).ok())
            .for_each(move |message| {
                *updater_state.lock().unwrap() = message;
                Ok(())
            })
            .map_err(|e| eprintln!("{:?}", e));

        state_updater.select(command_sender)
            .then(|_| Ok(()))
    }).map_err(|_| ());

    tokio::run(client);
    Ok(())
}

#[cfg(test)]
mod tests {
}
