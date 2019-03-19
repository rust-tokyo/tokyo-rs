use common::models::*;
use std::time::Instant;
use tokyo::{
    self,
    analyzer::Analyzer,
    strategy::{
        behavior::{Dodge, FireAt},
        condition::{Always, PlayerWithin},
        target::Target,
        PrioritizedBehavior, Strategy, StrategyNode,
    },
    Handler,
};

struct Player {
    analyzer: Analyzer,
    strategy: Strategy,

    current_behavior: PrioritizedBehavior,
}

impl Player {
    fn new() -> Self {
        Self {
            analyzer: Analyzer::new(),
            // Shoots at an enemy only if it's very close; otherwise keep dodging.
            strategy: Strategy::new(vec![
                (
                    Box::new(PlayerWithin { radius: 20.0 }),
                    Box::new(StrategyNode::Leaf(PrioritizedBehavior::with_high(
                        FireAt::once(Target::Closest),
                    ))),
                ),
                (
                    Box::new(Always {}),
                    Box::new(StrategyNode::Leaf(PrioritizedBehavior::with_medium(
                        Dodge {},
                    ))),
                ),
            ]),
            current_behavior: PrioritizedBehavior::new(),
        }
    }
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.analyzer.set_own_player_id(state.id);
        self.analyzer.push_state(&state.game_state, Instant::now());

        let next_command = self.current_behavior.behavior.next_command(&self.analyzer);
        if let Some(next_behavior) = self.strategy.next_behavior(&self.analyzer) {
            if next_behavior.priority > self.current_behavior.priority || next_command.is_none() {
                self.current_behavior = next_behavior;
                return self.current_behavior.behavior.next_command(&self.analyzer);
            }
        }
        next_command
    }
}

fn main() {
    println!("starting up...");
    tokyo::run(
        "403B9A2F-103F-4E43-8B52-1AC4870AA1E3",
        "H4CK TH3 PL4N3T",
        Player::new(),
    )
    .unwrap();
}