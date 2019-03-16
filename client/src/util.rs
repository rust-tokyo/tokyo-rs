use std::{time::{Instant, Duration}, collections::HashMap, ops::Sub};

use common::{models::{BulletState, PlayerState}, vec::Vec2};

pub trait AsSecsF32 {
    fn as_secs_f32(&self) -> f32;
}

impl AsSecsF32 for Duration {
    fn as_secs_f32(&self) -> f32 {
        self.as_nanos() as f32 / 1e9
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> f32 {
        other.sub(*self).magnitude()
    }

    pub fn angle_to(&self, other: &Position) -> f32 {
        other.sub(*self).angle()
    }

    pub fn velocity_to(&self, other: &Position, dt: Duration) -> Velocity {
        other.sub(*self).div(dt.as_secs_f32()).into2()
    }

    pub fn project(&self, vel: &Velocity, time: Duration) -> Position {
        self.add(vel.mul(time.as_secs_f32()))
    }
}

impl Vec2 for Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Clone, Debug)]
pub struct Velocity {
    pub dx: f32, // per second
    pub dy: f32, // per second
}

impl Velocity {
    pub fn with_angle_speed(angle: f32, speed: f32) -> Self {
        Velocity::with_angle(angle).mul(speed)
    }
}

impl Vec2 for Velocity {
    fn new(x: f32, y: f32) -> Self {
        Self { dx: x, dy: y }
    }

    fn x(&self) -> f32 {
        self.dx
    }

    fn y(&self) -> f32 {
        self.dy
    }
}

pub struct Player {
    pub id: u32,
    pub angle: f32,
    pub position: Position,
    pub trajectory: Trajectory,
    pub score_history: ScoreHistory,
    last_update: Instant,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: 0,
            angle: 0.0,
            position: Position::new(0.0, 0.0),
            trajectory: Trajectory::new(),
            score_history: ScoreHistory::new(),
            last_update: Instant::now(),
        }
    }

    pub fn with_state(state: &PlayerState, scoreboard: &HashMap<u32, u32>, time: Instant) -> Self {
        let position = Position::new(state.x, state.y);

        let mut trajectory = Trajectory::new();
        trajectory.push(position.clone(), time);

        let mut score_history = ScoreHistory::new();
        score_history.push(*scoreboard.get(&state.id).unwrap(), time);

        Self {
            id: state.id,
            angle: state.angle,
            position,
            trajectory,
            score_history,
            last_update: time,
        }
    }

    pub fn push_state(&mut self, state: &PlayerState, scoreboard: &HashMap<u32, u32>, time: Instant) {
        assert_eq!(self.id, state.id);

        self.angle = state.angle;
        self.position = Position::new(state.x, state.y);
        self.trajectory.push(self.position.clone(), time);
        self.score_history
            .push(*scoreboard.get(&state.id).unwrap(), time);
        self.last_update = time;
    }
}

pub struct Trajectory {
    pub positions: Vec<(Position, Instant)>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
        }
    }

    pub fn push(&mut self, position: Position, time: Instant) {
        self.positions.push((position, time));
    }

    pub fn last_position<'a>(&'a self) -> &'a Position {
        &self.positions.last().unwrap().0
    }

    pub fn last_velocity(&self) -> Velocity {
        let (last_position, last_time) = self.positions.last().unwrap();
        if let Some((prev_position, prev_time)) = self.positions.get(self.positions.len() - 2) {
            prev_position.velocity_to(last_position, *last_time - *prev_time)
        } else {
            // No idea, just return zeros.
            Velocity::zeros()
        }
    }

    // Some indication of the player's desire to move.
    pub fn ave_abs_velocity(&self) -> Velocity {
        if self.positions.len() < 2 {
            return Velocity::zeros();
        }

        let mut velocities = Vec::new();
        for ((prev_position, prev_time), (position, time)) in
            self.positions.iter().zip(self.positions.iter().skip(1))
        {
            velocities.push(prev_position.velocity_to(position, *time - *prev_time).abs());
        }

        Velocity {
            dx: velocities.iter().map(|v| v.dx).sum::<f32>() / velocities.len() as f32,
            dy: velocities.iter().map(|v| v.dy).sum::<f32>() / velocities.len() as f32,
        }
    }
}

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

pub struct Bullet {
    pub position: Position,
    pub velocity: Velocity,
    pub player_id: u32,
}

impl Bullet {
    pub fn new(state: &BulletState) -> Self {
        // TODO(ryo): Align with server implementation.
        const BULLET_SPEED: f32 = 1.0;

        Bullet {
            position: Position::new(state.x, state.y),
            velocity: Velocity::with_angle_speed(state.angle, BULLET_SPEED),
            player_id: state.player_id,
        }
    }
}
