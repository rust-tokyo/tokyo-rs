use std::time::Instant;
use common::models::*;
use tokyo::{self, Handler, strategy::{behavior::{Chase, Behavior, Sequence, FireAt}, target::Target}, analyzer::Analyzer};

#[derive(Default)]
struct Player {
    analyzer: Analyzer,
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.analyzer.set_own_player_id(state.id);
        self.analyzer.push_state(&state.game_state, Instant::now());
        if self.analyzer.is_dead() {
            return None;
        }

        // Keep chasing a player with the highest score and shoots it once it's
        // within reach.
        Sequence::two(
            Chase { target: Target::HighestScore, reach: 200.0 },
            FireAt::once(Target::HighestScore),
        ).next_command(&self.analyzer)
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("403B9A2F-103F-4E43-8B52-1AC4870AA1E3", "CHASER", Player::default()).unwrap();
}
