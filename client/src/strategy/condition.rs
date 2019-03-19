use crate::analyzer::Analyzer;
use std::time::{Duration, Instant};

pub trait Condition: Send {
    fn evaluate(&mut self, _: &Analyzer) -> bool;
}

pub struct Always;
impl Condition for Always {
    fn evaluate(&mut self, _: &Analyzer) -> bool {
        true
    }
}

pub struct And<T1, T2> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Condition, T2: Condition> Condition for And<T1, T2> {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        self.lhs.evaluate(analyzer) && self.rhs.evaluate(analyzer)
    }
}

pub struct Or<T1, T2> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Condition, T2: Condition> Condition for Or<T1, T2> {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        self.lhs.evaluate(analyzer) || self.rhs.evaluate(analyzer)
    }
}

pub struct Not<T> {
    inner: T,
}

impl<T: Condition> Condition for Not<T> {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        !self.inner.evaluate(analyzer)
    }
}

pub struct AtInterval {
    interval: Duration,
    next: Instant,
}

impl Condition for AtInterval {
    fn evaluate(&mut self, _: &Analyzer) -> bool {
        let now = Instant::now();
        if now >= self.next {
            self.next += self.interval;
            true
        } else {
            false
        }
    }
}

impl AtInterval {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            next: Instant::now(),
        }
    }
}

pub struct PlayerWithin {
    pub radius: f32
}

impl Condition for PlayerWithin {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        analyzer.players_within(self.radius).len() > 0
    }
}

pub struct PlayerWithHigherScore;
impl Condition for PlayerWithHigherScore {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        analyzer.player_highest_score().id != analyzer.own_player().id
    }
}

pub struct BulletWithin {
    pub radius: f32
}

impl Condition for BulletWithin {
    fn evaluate(&mut self, analyzer: &Analyzer) -> bool {
        analyzer.bullets_within(self.radius).len() > 0
    }
}
