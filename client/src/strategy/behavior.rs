use crate::{analyzer::Analyzer, geom::*, strategy::target::Target};
use common::models::{GameCommand, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED};
use rand::{thread_rng, Rng};
use std::{collections::VecDeque, fmt::Debug, time::Duration};

pub trait Behavior: Send + Debug {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand>;
    fn box_clone(&self) -> Box<Behavior>;
}

impl Clone for Box<Behavior> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Default for Box<Behavior> {
    fn default() -> Self {
        Box::new(Noop {})
    }
}

#[derive(Clone, Debug)]
pub struct Sequence {
    inner: VecDeque<Box<Behavior>>,
}

impl Behavior for Sequence {
    fn next_command(&mut self, analyzer: &Analyzer) -> Option<GameCommand> {
        while let Some(next) = self.inner.front_mut() {
            if let Some(command) = next.next_command(analyzer) {
                return Some(command);
            }
            self.inner.pop_front();
        }
        None
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl Sequence {
    pub fn new() -> Self {
        Sequence::with_slice(&[])
    }

    pub fn with_slice(behaviors: &[&Behavior]) -> Self {
        Self { inner: behaviors.into_iter().map(|b| b.box_clone()).collect::<VecDeque<_>>() }
    }
}

#[derive(Clone, Debug)]
pub struct Noop;

impl Behavior for Noop {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand> {
        None
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Forward {
    pub distance: f32,
}

impl Behavior for Forward {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand> {
        if self.distance > 0.0 {
            let next_move = PLAYER_MAX_SPEED.max(self.distance);
            self.distance -= next_move;
            Some(GameCommand::Forward(next_move))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl Forward {
    pub fn with_steps(steps: u32) -> Self {
        Self { distance: PLAYER_MAX_SPEED * steps as f32 }
    }
}

#[derive(Clone, Debug)]
pub struct Rotate {
    angle: Radian,
    margin: Radian,
}

impl Behavior for Rotate {
    fn next_command(&mut self, analyzer: &Analyzer) -> Option<GameCommand> {
        if (analyzer.own_player().angle.positive() - self.angle.positive()).abs() > self.margin {
            Some(GameCommand::Rotate(self.angle.positive().get()))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl Rotate {
    pub fn new(angle: Radian) -> Self {
        Self::with_margin_degrees(angle, 0.1)
    }

    pub fn with_margin_degrees(angle: Radian, margin_degrees: f32) -> Self {
        Self { angle, margin: Radian::degrees(margin_degrees) }
    }
}

#[derive(Clone, Debug)]
pub struct Fire {
    pub times: u32,
}

impl Behavior for Fire {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand> {
        if self.times > 0 {
            self.times -= 1;
            Some(GameCommand::Fire)
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl Fire {
    pub fn once() -> Self {
        Self { times: 1 }
    }
}

#[derive(Clone, Debug)]
pub struct FireAt {
    pub target: Target,
    pub times: u32,
    next: Sequence,
}

impl Behavior for FireAt {
    fn next_command(&mut self, analyzer: &Analyzer) -> Option<GameCommand> {
        if let Some(next_command) = self.next.next_command(analyzer) {
            return Some(next_command);
        }

        if self.times > 0 {
            if let Some(target) = self.target.get(analyzer) {
                self.times -= 1;
                let angle = analyzer.own_player().angle_to(target);
                self.next = Sequence::with_slice(&[
                    &Rotate::with_margin_degrees(angle, 5.0),
                    &Fire::once(),
                ]);
                return self.next.next_command(analyzer);
            }
        }
        None
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl FireAt {
    pub fn new(target: Target) -> Self {
        Self::with_times(target, 1)
    }

    pub fn with_times(target: Target, times: u32) -> Self {
        Self { target, times, next: Sequence::new() }
    }
}

#[derive(Clone, Debug)]
struct Random;

impl Behavior for Random {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand> {
        let mut rng = thread_rng();
        match rng.gen_range(0, 4) {
            0 => None,
            1 => Some(GameCommand::Rotate(rng.gen_range(0.0, 2.0 * std::f32::consts::PI))),
            2 => Some(GameCommand::Forward(rng.gen_range(PLAYER_MIN_SPEED, PLAYER_MAX_SPEED))),
            3 => Some(GameCommand::Fire),
            _ => unreachable!(),
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Chase {
    pub target: Target,
    pub distance: f32,
}

impl Behavior for Chase {
    fn next_command(&mut self, analyzer: &Analyzer) -> Option<GameCommand> {
        if let Some(target) = self.target.get(analyzer) {
            let distance_to_target = analyzer.own_player().distance(target);
            if distance_to_target > self.distance {
                let angle = analyzer.own_player().angle_to(target);
                return Sequence::with_slice(&[
                    &Rotate::with_margin_degrees(angle, 10.0),
                    &Forward::with_steps(1),
                ])
                .next_command(analyzer);
            }
        }
        None
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Dodge;

impl Behavior for Dodge {
    fn next_command(&mut self, analyzer: &Analyzer) -> Option<GameCommand> {
        if let Some(bullet) = analyzer.bullets_colliding(Duration::from_secs(3)).iter().next() {
            let angle = bullet.velocity.tangent();
            Sequence::with_slice(&[
                &Rotate::with_margin_degrees(angle, 30.0),
                &Forward::with_steps(1),
            ])
            .next_command(analyzer)
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}
