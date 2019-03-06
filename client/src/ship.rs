use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::{GameCommand, GameState};

pub struct Ship {
    engine: Box<Engine>,
    scanner: Box<Scanner>,
    strategy: Strategy,
    next_commands: VecDeque<GameCommand>,
}

impl Ship {
    pub fn with(engine: Box<Engine>, scanner: Box<Scanner>) -> Self {
        Self {
            engine,
            scanner,
            strategy: Vec::new(),
            next_commands: VecDeque::new(),
        }
    }

    pub fn push_state(&mut self, state: GameState) {
        // TODO(ryo): Implement.
    }

    pub fn set_strategy(&mut self, strategy: Strategy) {
        self.strategy = strategy;
    }

    pub fn next_command(&mut self) -> Option<GameCommand> {
        if let Some(next_command) = self.next_commands.pop_front() {
            return Some(next_command);
        }

        match self.next_action() {
            Action::ChaseFor(target) => {
                let velocity = self.engine.throttle();
                let angle = self.scanner.angle_to_chase_for(target, velocity);

                self.next_commands
                    .push_back(GameCommand::Throttle(velocity));
                Some(GameCommand::Rotate(angle))
            }
            // TODO(ryo, player): Implement more actions.
            _ => None,
        }
    }

    fn next_action(&mut self) -> Action {
        for (action, condition) in &mut self.strategy {
            if condition.evaluate(&self.scanner) {
                return action.clone();
            }
        }
        Action::Hold
    }
}

#[derive(Clone)]
pub enum Action {
    /// Do nothing.
    Hold,

    /// Do something random.
    Random,

    /// Chase for the target player.
    ChaseFor(u32),

    /// Fire at the current angle.
    Fire,

    /// Fire at the target player.
    FireAt(u32),

    /// Try to minimize number of hits.
    Dodge,
}

pub trait Condition: Send {
    fn evaluate(&mut self, scanner: &Box<Scanner>) -> bool;
}

pub struct Always;
impl Condition for Always {
    fn evaluate(&mut self, _: &Box<Scanner>) -> bool {
        true
    }
}

pub struct AtInterval {
    interval: Duration,
    next: Instant,
}

impl Condition for AtInterval {
    fn evaluate(&mut self, _: &Box<Scanner>) -> bool {
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

// TODO(ryo, player): Implement more interesting conditions.

/// The first Action that has Condition met will be chosen as the strategy of the
/// current turn.
pub type Strategy = Vec<(Action, Box<Condition>)>;

pub trait Engine: Send {
    fn throttle(&mut self) -> f32;
}

pub struct NormalEngine;
impl Engine for NormalEngine {
    fn throttle(&mut self) -> f32 {
        1.0
    }
}

pub struct CustomEngine {
    throttle: f32,
}

impl Engine for CustomEngine {
    fn throttle(&mut self) -> f32 {
        self.throttle
    }
}

// TODO(player): Pick a right set of components to create a custom engine.
// TODO(ryo): Implement predefined components that can be combined with iterator.
impl CustomEngine {
    pub fn with<F1, F2, F3, IT>(component1: F1, component2: F2, component3: F3) -> Self
    where
        F1: Fn() -> IT,
        F2: Fn(f32) -> Option<f32>,
        F3: Fn(f32, f32) -> f32,
        IT: Iterator<Item = f32>,
    {
        Self {
            throttle: component1()
                .flat_map(component2)
                .fold(0.0, component3)
        }
    }
}

pub trait Storage {
    /// Stores the current state in the storage.
    fn push_state(&mut self, state: GameState);

    /// Gets the current state.
    fn state<'a>(&'a self) -> Option<&'a GameState>;

    /// Gets the historical state by specifying num of ticks to go back.
    fn past_state<'a>(&'a self, ticks_ago: usize) -> Option<&'a GameState>;
}

/// Poor man's storage can only store the current state.
pub struct FloppyDisk {
    state: Option<GameState>,
}

impl Storage for FloppyDisk {
    fn push_state(&mut self, state: GameState) {
        self.state = Some(state);
    }

    fn state<'a>(&'a self) -> Option<&'a GameState> {
        if let Some(state) = &self.state {
            Some(&state)
        } else {
            None
        }
    }

    fn past_state<'a>(&'a self, generations_ago: usize) -> Option<&'a GameState> {
        if generations_ago == 0 {
            self.state()
        } else {
            None
        }
    }
}

impl FloppyDisk {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

// TODO(player): Implement a better storage than floppy disk.

pub trait Scanner: Send {
    fn velocity_of(&self, target: u32) -> f32 {
        // TODO(ryo): Give default implementation.
        unimplemented!();
    }

    fn angle_to_chase_for(&self, target: u32, own_velocity: f32) -> f32 {
        // TODO(ryo): Give default implementation.
        unimplemented!();
    }

    fn estimate_num_hits(&self, velocity: f32, angle: f32, duration: Duration) -> u32 {
        // TODO(ryo): Give default implementation.
        unimplemented!();
    }
}

// TODO(player): Implement your Scanner.
