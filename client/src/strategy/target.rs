use crate::analyzer::{Analyzer, player::Player};
use std::time::Duration;

/// Target player.
#[derive(Clone, Debug)]
pub enum Target {
    Id(u32),

    /// Player closest to yourself.
    Closest,

    /// Player that is least moving.
    LeastMoving,

    /// Player with the height score earned.
    HighestScore,

    /// Player with the height projected score after the specified duration.
    HighestScoreAfter(Duration),
}

impl Target {
    pub fn get<'a>(&self, analyzer: &'a Analyzer) -> Option<&'a Player> {
        match self {
            Target::Id(id) => analyzer.player(*id),
            Target::Closest => analyzer.player_closest(),
            Target::LeastMoving => analyzer.player_least_moving(),
            Target::HighestScore => analyzer.player_highest_score(),
            Target::HighestScoreAfter(after) => analyzer.player_highest_score_after(*after),
        }
    }
}
