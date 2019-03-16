use common::models::*;
use tokyo::{self, Handler};

struct Player {
    id: Option<u32>,
    angle: f32,
    counter: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            id: None,
            angle: 0.0,
            counter: 0,
        }
    }
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        if self.id.is_none() {
            self.id = Some(state.id);
        }

        let angle = self.angle;
        self.angle += 0.01;

        self.counter += 1;

        Some(match self.counter % 3 {
            0 => GameCommand::Rotate(angle),
            1 => GameCommand::Fire,
            _ => GameCommand::Forward(10.0),
        })
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("44848DB2-3778-431F-B3F9-61F293C65CC7", Player::new()).unwrap();
}
