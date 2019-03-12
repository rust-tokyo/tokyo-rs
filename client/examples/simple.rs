use common::models::*;
use tokyo::{
    self,
    ship::{Computer, FloppyDisk, NormalEngine, Ship, Storage, StorageAccess},
};

struct Player {
    id: Option<u32>,
    // TODO(player): Customize your ship.
    ship: Ship<NormalEngine, OldComputer>,
}

impl Player {
    fn new() -> Self {
        Self {
            id: None,
            ship: Ship::with(NormalEngine {}, OldComputer::new()),
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
    tokyo::run("7DEA6163-7532-4420-9ECC-10773347DCAE", Player::new()).unwrap();
}
