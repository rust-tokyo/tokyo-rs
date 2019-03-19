use std::time::Instant;
use common::models::*;
use tokyo::{self, Handler, strategy::{behavior::{Chase, Behavior, Sequence, FireAt}, target::Target}, analyzer::Analyzer};

#[derive(Default)]
struct Player {
    analyzer: Analyzer,
    current_behavior: Box<Behavior>,
}

fn chase() -> Box<Behavior> {
    // Behavior to keep chasing the target (in this case, the player with
    // the highest score.) It yields to the next behavior when the distance
    // to the player is less than 200.0.
    let chase = Chase { target: Target::HighestScore, distance: 200.0 };

    // Behavior to fire at the target player twice.
    let fire = FireAt::with_times(Target::HighestScore, 2);

    // A sequence of behaviors: chase and then fire.
    Box::new(Sequence::with_slice(&[&chase, &fire]))
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.analyzer.set_own_player_id(state.id);
        self.analyzer.push_state(&state.game_state, Instant::now());
        if self.analyzer.is_dead() {
            return None;
        }

        if let Some(command) = self.current_behavior.next_command(&self.analyzer) {
            Some(command)
        } else {
            self.current_behavior = chase();
            self.current_behavior.next_command(&self.analyzer)
        }
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("403B9A2F-103F-4E43-8B52-1AC4870AA1E3", "CHASER", Player::default()).unwrap();
}
