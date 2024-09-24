use std::fmt::{self, Debug};
use std::ops::{Add, Div, Mul, Sub};

use super::interval::Interval;
use super::min::min_f64;
use super::random::random_f64;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    // Unit vector
    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        if length == 0.0 {
            // Handle zero-length vector appropriately
            Vec3::new(0.0, 0.0, 0.0) // Or you could panic! or return an Option<Vec3>
        } else {
            *self / length
        }
    }

    // Dot product
    pub fn dot(&self, b: &Vec3) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    // Cross product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn random() -> Self {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn inverted(&self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn random_interval(interval: Interval) -> Self {
        Vec3::new(interval.random(), interval.random(), interval.random())
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_interval(Interval::new(-1.0, 1.0));
            let lensq = p.length_squared();
            // Use a more practical threshold to avoid floating-point precision issues
            if lensq > 1e-8 && lensq <= 1.0 {
                return p / lensq.sqrt(); // Normalize the vector to length 1
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let interval = Interval::new(-1.0, 1.0);
            let p = Vec3::new(interval.random(), interval.random(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere.inverted()
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        return *self - 2. * self.dot(normal) * *normal;
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = min_f64(self.inverted().dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = (-f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared()))) * *normal;
        return r_out_perp + r_out_parallel;
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

// Add
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        self + Vec3::new(other, other, other)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        other + self
    }
}

// Sub
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        self - Vec3::new(other, other, other)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        other - self
    }
}

// Mul
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        self * Vec3::new(other, other, other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

// Div
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self / Vec3::new(other, other, other)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        other / self
    }
}

pub type Point3 = Vec3;

pub type Color = Vec3;

// Implementing Display for vec3 to replace std::ostream
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}
