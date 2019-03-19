use euclid::{Angle, Point2D, Vector2D};
use std::time::Duration;

pub type Point = Point2D<f32>;
pub type Vector = Vector2D<f32>;
pub type Radian = Angle<f32>;

pub trait AsSecsF32 {
    fn as_secs_f32(&self) -> f32;
}

impl AsSecsF32 for Duration {
    fn as_secs_f32(&self) -> f32 {
        self.as_nanos() as f32 / 1e9
    }
}

pub trait PointExt {
    fn point(&self) -> &Point;

    fn distance(&self, other: &PointExt) -> f32 {
        (*other.point() - *self.point()).length()
    }

    fn angle_to(&self, other: &PointExt) -> Radian {
        (*other.point() - *self.point()).angle_from_x_axis()
    }

    fn velocity_to(&self, other: &PointExt, dt: Duration) -> Vector {
        (*other.point() - *self.point()) / dt.as_secs_f32()
    }

    fn project(&self, vel: &Vector, dt: Duration) -> Point {
        *self.point() + *vel * dt.as_secs_f32()
    }
}

impl PointExt for Point {
    fn point(&self) -> &Point {
        self
    }
}

pub trait VectorExt {
    fn vector(&self) -> &Vector;

    fn with_angle(angle: f32) -> Vector {
        Vector::new(angle.cos(), angle.sin())
    }

    fn tangent(&self) -> Radian {
        (self.vector().angle_from_x_axis() + Radian::frac_pi_2()).positive()
    }
}

impl VectorExt for Vector {
    fn vector(&self) -> &Vector {
        self
    }
}

pub trait RadianExt {
    fn radian(&self) -> &Radian;

    fn new(radians: f32) -> Radian {
        Radian::radians(radians)
    }

    fn abs(&self) -> Radian {
        Radian::radians(self.radian().radians.abs())
    }
}

impl RadianExt for Radian {
    fn radian(&self) -> &Radian {
        self
    }
}
