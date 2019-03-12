use common::models::*;
use tokyo::{
    self,
    ship::{Computer, FloppyDisk, NormalEngine, Ship, Storage, StorageAccess},
};

struct Player {
    id: Option<u32>,
    // TODO(player): Customize your ship.
    angle: f32,
    counter: u32,
    ship: Ship<NormalEngine, OldComputer>,
}

impl Player {
    fn new() -> Self {
        Self {
            id: None,
            angle: 0.0,
            counter: 0,
            ship: Ship::with(NormalEngine {}, OldComputer::new()),
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
    }
}

struct OldComputer {
    storage: FloppyDisk,
}

impl StorageAccess for OldComputer {
    fn storage<'a>(&'a self) -> &'a Storage {
        &self.storage
    }

    fn storage_mut<'a>(&'a mut self) -> &'a mut Storage {
        &mut self.storage
    }
}

impl Computer for OldComputer {}

impl OldComputer {
    fn new() -> Self {
        Self {
            storage: FloppyDisk::new(),
        }
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("44848DB2-3778-431F-B3F9-61F293C65CC7", Player::new()).unwrap();
}
