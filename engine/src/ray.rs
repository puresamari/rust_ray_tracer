use serde::{Deserialize, Serialize};

use math::vec3::{Point3, Vec3};

// Ray 𝐏(𝑡)=𝐀+𝑡𝐛
// P is a 3d point along a 3d line
// A is the origin of the line
// b is the direction of the line
// t is a real number
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,

    time: f64,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self::new_with_time(origin, direction, 0.0)
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }
}
