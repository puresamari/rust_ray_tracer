use crate::math::vec3::{Point3, Vec3};

// Ray 𝐏(𝑡)=𝐀+𝑡𝐛
// P is a 3d point along a 3d line
// A is the origin of the line
// b is the direction of the line
// t is a real number
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
