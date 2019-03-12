use common::models::*;
use tokyo::{
    self,
    ship::{NormalEngine, Scanner, Ship},
};

struct Player {
    id: Option<u32>,
    // TODO(player): Customize your ship.
    ship: Ship,
}

impl Player {
    fn new() -> Self {
        Self {
            id: None,
            ship: Ship::with(
                Box::new(NormalEngine {}),
                Box::new(SimpleScanner {}),
            ),
        }
    }
}

impl tokyo::Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        println!("{:#?}", state);

        if self.id.is_none() {
            self.id = Some(state.id);
        }

        self.ship.push_state(state.game_state.clone());
        self.ship.next_command()
    }
}

struct SimpleScanner;
impl Scanner for SimpleScanner {
}

fn main() {
    println!("starting up...");
    tokyo::run("7DEA6163-7532-4420-9ECC-10773347DCAE", Player::new()).unwrap();
}
