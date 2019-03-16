use std::{
    collections::VecDeque,
    ops::Sub,
    time::{Duration, Instant},
};

use common::models::{ClientState, GameCommand, MAX_SPEED};

use crate::radar::Radar;

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
            Behavior::ChaseFor(target) => {
                let angle = self.radar.angle_to(target);
                if self.radar.own_player().angle.sub(angle).abs() > 10.0 {
                    Some(GameCommand::Rotate(angle))
                } else {
                    Some(GameCommand::Forward(MAX_SPEED))
                }
            }
            // TODO(ryo, player): Implement more behaviors.
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
