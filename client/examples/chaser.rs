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
    tokyo::run("403B9A2F-103F-4E43-8B52-1AC4870AA1E3", Player::new()).unwrap();
}
