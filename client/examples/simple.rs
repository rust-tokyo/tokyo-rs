use common::models::*;
use tokyo::{
    self,
    ship::{NormalEngine, Scanner, Ship},
};

struct Player {
    id: Option<u32>,
    // TODO(player): Customize your ship.
    ship: Ship,
    angle: f32,
    counter: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            id: None,
            ship: Ship::with(
                Box::new(NormalEngine {}),
                Box::new(SimpleScanner {}),
            ),
            angle: 0.0,
            counter: 0,
        }
    }
}

impl tokyo::Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        // println!("{:#?}", state);

        if self.id.is_none() {
            self.id = Some(state.id);
        }

        self.ship.push_state(state.game_state.clone());
        // self.ship.next_command()

        let angle = self.angle;
        self.angle += 0.01;

        self.counter += 1;

        Some(match self.counter % 3 {
            0 => GameCommand::Rotate(angle),
            1 => GameCommand::Fire,
            _ => GameCommand::Forward(10.0)
        })

        // if self.counter % 3 == 0 {
        //     Some(GameCommand::Rotate(angle))
        // } else if self.counter % 3 == 1 {
        //     Some()
        // } else {
        //     Some(GameCommand::Forward(10.0))
        // }
    }
}

struct SimpleScanner;
impl Scanner for SimpleScanner {
}

fn main() {
    println!("starting up...");
    tokyo::run("44848DB2-3778-431F-B3F9-61F293C65CC7", Player::new()).unwrap();
}
