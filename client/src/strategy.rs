use crate::{geom::*, radar::Radar};
use common::models::{ClientState, GameCommand, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED};
use rand::{thread_rng, Rng};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

type BehaviorVec = Vec<(Behavior, Box<Condition>)>;

pub struct Strategy {
    behaviors: BehaviorVec,
    next_commands: VecDeque<GameCommand>,
    radar: Radar,
}

impl Strategy {
    pub fn new(behaviors: BehaviorVec) -> Self {
        Self {
            behaviors,
            next_commands: VecDeque::new(),
            radar: Radar::new(),
        }
    }

    pub fn push_state(&mut self, state: &ClientState) {
        self.radar.set_own_player_id(state.id);
        self.radar.push_state(&state.game_state, Instant::now());
    }

    pub fn next_command(&mut self) -> Option<GameCommand> {
        if let Some(next_command) = self.next_commands.pop_front() {
            return Some(next_command);
        }

        match self.next_behavior() {
            Behavior::Random => {
                let mut rng = thread_rng();
                match rng.gen_range(0, 4) {
                    0 => None,
                    1 => Some(GameCommand::Rotate(
                        rng.gen_range(0.0, 2.0 * std::f32::consts::PI),
                    )),
                    2 => Some(GameCommand::Forward(
                        rng.gen_range(PLAYER_MIN_SPEED, PLAYER_MAX_SPEED),
                    )),
                    3 => Some(GameCommand::Fire),
                    _ => unreachable!(),
                }
            }
            Behavior::ChaseFor(target) => {
                let angle = self.radar.angle_to(target);
                if (self.radar.own_player().angle - angle).abs().get() > 10.0 {
                    Some(GameCommand::Rotate(angle.positive().get()))
                } else {
                    Some(GameCommand::Forward(PLAYER_MAX_SPEED))
                }
            }
            Behavior::FireAt(target) => {
                let angle = self.radar.angle_to(target);
                if (self.radar.own_player().angle - angle).abs().get() > 1.0 {
                    Some(GameCommand::Rotate(angle.positive().get()))
                } else {
                    Some(GameCommand::Fire)
                }
            }
            Behavior::Dodge => {
                // The provided implementation only avoids a single collision
                // occurrence. It also doesn't check the consequence of
                // performing the dodge action.
                let dodge_until = Instant::now() + Duration::from_secs(1);
                if let Some(bullet) = self.radar.bullets_to_collide(dodge_until).iter().next() {
                    // Move 2 step forward to the direction to move away from the bullet trajectory.
                    self.next_commands
                        .push_back(GameCommand::Forward(PLAYER_MAX_SPEED));
                    self.next_commands
                        .push_back(GameCommand::Forward(PLAYER_MAX_SPEED));
                    Some(GameCommand::Rotate(
                        bullet.velocity.tangent().positive().get(),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn next_behavior(&mut self) -> Behavior {
        for (behavior, condition) in &mut self.behaviors {
            if condition.evaluate() {
                return behavior.clone();
            }
        }
        Behavior::Hold
    }
}

#[derive(Clone)]
pub enum Behavior {
    /// Do nothing.
    Hold,

    /// Do something random.
    Random,

    /// Keep chasing for the target player.
    ChaseFor(u32),

    /// Fire at the target player.
    FireAt(u32),

    /// Try to minimize number of hits while in this mode.
    Dodge,
}

pub trait Condition: Send {
    fn evaluate(&mut self) -> bool;
}

pub struct Always;
impl Condition for Always {
    fn evaluate(&mut self) -> bool {
        true
    }
}

pub struct AtInterval {
    interval: Duration,
    next: Instant,
}

impl Condition for AtInterval {
    fn evaluate(&mut self) -> bool {
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
    pub fn with(interval: Duration) -> Self {
        Self {
            interval,
            next: Instant::now(),
        }
    }
}
