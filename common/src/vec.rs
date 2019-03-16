/// Traits with basic vector math.

use std::f32::consts::PI;

pub trait Vec2: Sized {
    fn new(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn with_angle(angle: f32) -> Self {
        Self::new(
            angle.cos(),
            angle.sin()
        )
    }

    fn zeros() -> Self {
        Self::new(0.0, 0.0)
    }

    fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }

    fn angle(&self) -> f32 {
        self.y().atan2(self.x())
    }

    fn tangent(&self) -> f32 {
        let tangent = self.angle() + PI * 0.5;
        if tangent > PI * 2.0 {
            tangent - PI * 2.0
        } else {
            tangent
        }
    }

    fn add<T: Vec2>(self, rhs: T) -> Self {
        Self::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
        )
    }

    fn sub<T: Vec2>(self, rhs: T) -> Self {
        Self::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
        )
    }

    fn mul(self, rhs: f32) -> Self {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
        )
    }

    fn div(self, rhs: f32) -> Self {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
        )
    }

    fn abs(self) -> Self {
        Self::new(self.x().abs(), self.y().abs())
    }

    // Note: std::convert::Into can not be implemented for a type parameter.
    fn into2<T: Vec2>(&self) -> T {
        T::new(self.x(), self.y())
    }
}

pub trait Vec3: Sized {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;

    fn zeros() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    fn add<T: Vec3>(self, rhs: T) -> Self {
        Self::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }

    fn sub<T: Vec3>(self, rhs: T) -> Self {
        Self::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }

    fn mul(self, rhs: f32) -> Self {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }

    fn div(self, rhs: f32) -> Self {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
        )
    }

    fn abs(self) -> Self {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    // Note: std::convert::Into can not be implemented for a type parameter.
    fn into2<T: Vec3>(&self) -> T {
        T::new(self.x(), self.y(), self.z())
    }
}