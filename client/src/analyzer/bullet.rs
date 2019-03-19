use crate::{analyzer::Positioned, geom::*};
use common::models::{BulletState, BULLET_SPEED};
use std::time::Duration;

pub struct Bullet {
    pub position: Point,
    pub velocity: Vector,
    pub player_id: u32,
}

impl Bullet {
    pub fn new(state: &BulletState) -> Self {
        Bullet {
            position: Point::new(state.x, state.y),
            velocity: Vector::with_angle(state.angle) * BULLET_SPEED,
            player_id: state.player_id,
        }
    }

    pub fn project_position(&self, after: Duration) -> Point {
        self.position.project(&self.velocity, after)
    }
}

impl Positioned for Bullet {
    fn position(&self) -> Point {
        self.position
    }
}
