use crate::{
    analyzer::{player::Player, Analyzer},
    geom::*,
    models::{GameCommand, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED},
};
use rand::{thread_rng, Rng};
use std::{collections::VecDeque, fmt::Debug, time::Duration};

/// `Behavior` trait abstracts an action or a series of actions that a `Player`
/// can take. It may be useful if you want to model a complex behavior, that
/// spans multiple ticks, or whose interpretation changes dynamically. You can
/// use `Sequence::with_slice()` to combine multiple behaviors.
///
/// Some `Behavior`s take `Target` as an argument to dynamically specify which
/// player to act against. See its documentation for details (later in this
/// file).
///
/// # Examples
///
/// A stateful usage of `Behavior`.
///
/// ```
/// impl Handlar for Player {
///     fn tick(...) {
///         self.analyzer.push_state(state, Instant::now());
///
///         if let Some(next_command) = self.current_behavior.next_command(&self.analyzer) {
///             return Some(next_command);
///         }
///
///         // Creates a Behavior and stores it in the Player struct, as we need to
///         // persist the state across ticks and keep track of the number of times it
///         // fired.
///         self.current_behavior = Self::next_behavior();
///
///         self.current_behavior.next_command(&analyzer)
///     }
///
///     fn next_behavior() -> Sequence {
///         // Behavior to keep chasing the target (in this case, the player with
///         // the highest score.) It yields to the next behavior when the distance
///         // to the player is less than 200.0.
///         let chase = Chase { target: Target::HighestScore, distance: 200.0 };
///
///         // Behavior to fire at the target player twice.
///         let fire = FireAt::with_times(Target::HighestScore, 2);
///
///         // A sequence of behaviors: chase and then fire twice.
///         Sequence::with_slice(&[&chase, &fire])
///     }
/// }
/// ```
///
/// A stateless usage of `Behavior`.
///
/// ```
/// impl Handlar for Player {
///     fn tick(...) {
///         self.analyzer.push_state(state, Instant::now());
///
///         // Find one of the bullets that are colliding within a second.
///         if let Some(bullet) = self.analyzer.bullets_colliding(Duration::from_secs(1)).next() {
///             let angle = bullet.velocity.tangent();
///
///             // Try to dodge from the bullet by moving to a direction roughly
///             // perpendicular to the bullet velocity.
///             let dodge = Sequence::with_slice(&[
///                 &Rotate::with_margin_degrees(angle, 30.0),
///                 &Forward::with_steps(1),
///             ]);
///
///             // This Behavior works without persisting it somewhere for the next tick() as
///             // in the previous example. At the next tick(), Rotate behavior will most likely
///             // return None, proceeding immediately to the Forward behavior. If the situation
///             // changes e.g. the bullet hit someone else, or there are other bullets
///             // colliding, then it may take the Rotate behavior again, but it's likely an
///             // optimal adjustment (assuming your logic of selecting a bullet to dodge is
///             // stable.)
///             return dodge.next_command(&self.analyzer);
///         }
///         None
///     }
/// }
/// ```
pub trait Behavior: Send + Debug {
    // Returns the next `GameCommand` to achieve this `Behavior`. None if there
    // is nothing more to do.
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand>;

    // `Clone` does not work nicely with `Box` yet, so you'll need to implement
    // this method manually for each struct.
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

/// `Sequence` represents a series of `Behavior`s. The first
/// `Behavior::next_command()` is repeatedly called until it yields `None`, and
/// then it moves to the second `Behavior`, and so forth.
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

/// A `Behavior` to do nothing.
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

/// A `Behavior` to keep yielding `GameCommand::Throttle` commands until it
/// travels the `distance`.
#[derive(Clone, Debug)]
pub struct Forward {
    pub distance: f32,
}

impl Behavior for Forward {
    fn next_command(&mut self, _: &Analyzer) -> Option<GameCommand> {
        if self.distance > 0.0 {
            let next_move = PLAYER_MAX_SPEED.max(self.distance);
            self.distance -= next_move;
            Some(GameCommand::Throttle(next_move))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Behavior> {
        Box::new(self.clone())
    }
}

impl Forward {
    /// Creates a new `Forward` to move `steps`. Each step is the maximum
    /// distance one can travel by a tick.
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
    times: u32,
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
    pub fn new() -> Self {
        Self::with_times(1)
    }

    pub fn with_times(times: u32) -> Self {
        Self { times }
    }
}

#[derive(Clone, Debug)]
pub struct FireAt {
    target: Target,
    times: u32,
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
                self.next =
                    Sequence::with_slice(&[&Rotate::with_margin_degrees(angle, 5.0), &Fire::new()]);
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
            2 => Some(GameCommand::Throttle(rng.gen_range(PLAYER_MIN_SPEED, PLAYER_MAX_SPEED))),
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
        if let Some(bullet) = analyzer.bullets_colliding(Duration::from_secs(3)).next() {
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
