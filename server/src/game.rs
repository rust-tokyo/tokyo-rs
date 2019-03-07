use common::models::{GameState, PlayerState};
use std::collections::HashMap;

pub struct Game {
    pub state: GameState,
    player_id_counter: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState {
                players: vec![],
                bullets: vec![],
                scoreboard: HashMap::new(),
            },
            player_id_counter: 0,
        }
    }

    pub fn add_player() -> u32 {
        // players.push()
        2
    }

    pub fn init(&mut self) {
        self.state.players.push(PlayerState {
            id: 0,
            angle: 0.0,
            x: 0,
            y: 0,
        });
    }

    pub fn tick(&self, _dt: f32) {}
}
