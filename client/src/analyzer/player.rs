use crate::{
    analyzer::{bullet::Bullet, ANALYSIS_INTERVAL},
    geom::*,
};
use common::models::{PlayerState, BULLET_RADIUS, PLAYER_RADIUS};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub angle: Radian,
    pub position: Point,
    pub trajectory: Trajectory,
    pub score_history: ScoreHistory,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: 0,
            angle: Radian::zero(),
            position: Point::zero(),
            trajectory: Trajectory::new(),
            score_history: ScoreHistory::new(),
        }
    }

    pub fn with_state(state: &PlayerState, scoreboard: &HashMap<u32, u32>, time: Instant) -> Self {
        let position = Point::new(state.x, state.y);

        let mut trajectory = Trajectory::new();
        trajectory.push(position.clone(), time);

        let mut score_history = ScoreHistory::new();
        score_history.push(*scoreboard.get(&state.id).unwrap_or(&0), time);

        Self { id: state.id, angle: Radian::new(state.angle), position, trajectory, score_history }
    }

    pub fn push_state(
        &mut self,
        state: &PlayerState,
        scoreboard: &HashMap<u32, u32>,
        time: Instant,
    ) {
        assert_eq!(self.id, state.id);

        self.angle = Radian::new(state.angle);
        self.position = Point::new(state.x, state.y);
        self.trajectory.push(self.position.clone(), time);
        self.score_history.push(*scoreboard.get(&state.id).unwrap_or(&0), time);
    }

    pub fn score(&self) -> u32 {
        self.score_history.last_score()
    }

    pub fn is_colliding_after(&self, bullet: &Bullet, after: Duration) -> bool {
        self.position.distance(&bullet.project_position(after)) < BULLET_RADIUS + PLAYER_RADIUS
    }

    pub fn is_colliding_during(&self, bullet: &Bullet, during: Duration) -> bool {
        let num_analysis = (during.as_millis() / ANALYSIS_INTERVAL.as_millis()) as u32;
        (1..num_analysis + 1)
            .map(|tick| self.is_colliding_after(bullet, ANALYSIS_INTERVAL * tick))
            .any(|hit| hit)
    }
}

impl PointExt for Player {
    fn point(&self) -> &Point {
        &self.position
    }
}

#[derive(Debug)]
pub struct Trajectory {
    pub positions: Vec<(Point, Instant)>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self { positions: Vec::new() }
    }

    pub fn push(&mut self, position: Point, time: Instant) {
        self.positions.push((position, time));
    }

    pub fn last_position<'a>(&'a self) -> &'a Point {
        &self.positions.last().unwrap().0
    }

    pub fn last_velocity(&self) -> Vector {
        let (last_position, last_time) = self.positions.last().unwrap();
        if let Some((prev_position, prev_time)) = self.positions.get(self.positions.len() - 2) {
            prev_position.velocity_to(last_position, *last_time - *prev_time)
        } else {
            // No idea, just return zeros.
            Vector::zero()
        }
    }

    // Some indication of the player's desire to move.
    pub fn ave_abs_velocity(&self) -> Vector {
        let (items, sum) = self
            .positions
            .iter()
            .zip(self.positions.iter().skip(1))
            .map(|((prev_position, prev_time), (position, time))| {
                prev_position.velocity_to(position, *time - *prev_time).abs()
            })
            .fold((0, Vector::zero()), |acc, next| (acc.0 + 1, acc.1 + next));

        if items == 0 {
            Vector::zero()
        } else {
            sum / items as f32
        }
    }
}

#[derive(Debug)]
pub struct ScoreHistory {
    inner: Vec<(u32, Instant)>,
}

impl ScoreHistory {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, score: u32, time: Instant) {
        self.inner.push((score, time));
    }

    pub fn last_score(&self) -> u32 {
        self.inner.last().unwrap().0
    }

    pub fn score_since(&self, since: Instant) -> u32 {
        let start_score = self
            .inner
            .iter()
            .rev()
            .find_map(|(score, time)| if *time <= since { Some(*score) } else { None })
            .unwrap_or(0u32);
        self.last_score() - start_score
    }

    pub fn project(&self, after: Duration) -> u32 {
        let past_duration = Duration::from_secs(10); // configurable
        let past_score = self.score_since(Instant::now() - past_duration);
        self.last_score()
            + (past_score as f32 * (after.as_millis() as f32 / past_duration.as_millis() as f32))
                as u32
    }
}
