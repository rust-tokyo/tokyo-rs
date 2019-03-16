use common::models::*;
use tokyo::{self, Handler, strategy::*};

struct Player {
    strategy: Strategy,
}

impl Player {
    fn new() -> Self {
        Self {
            strategy: Strategy::new(vec![
                (Behavior::ChaseFor(0), Box::new(Always {})),
            ]),
        }
    }
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.strategy.push_state(state);
        self.strategy.next_command()
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("44848DB2-3778-431F-B3F9-61F293C65CC7", Player::new()).unwrap();
}
