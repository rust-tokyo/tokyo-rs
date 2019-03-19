use std::time::Instant;
use common::models::*;
use tokyo::{self, Handler, strategy::{behavior::{Chase, Behavior}, target::Target}, analyzer::Analyzer};

struct Player {
    analyzer: Analyzer,
    behavior: Chase,
}

impl Player {
    fn new() -> Self {
        Self {
            analyzer: Analyzer::new(),
            behavior: Chase { target: Target::HighestScore },
        }
    }
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.analyzer.set_own_player_id(state.id);
        self.analyzer.push_state(&state.game_state, Instant::now());

        self.behavior.next_command(&self.analyzer)
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("403B9A2F-103F-4E43-8B52-1AC4870AA1E3", "H4CK TH3 PL4N3T", Player::new()).unwrap();
}
