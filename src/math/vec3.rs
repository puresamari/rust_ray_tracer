use std::fmt;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};

use super::interval::Interval;
use super::random::{random_f64, random_f64_in_interval};

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
        *self / self.length()
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
        Vec3::new(
            random_f64_in_interval(&interval),
            random_f64_in_interval(&interval),
            random_f64_in_interval(&interval),
        )
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

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere.inverted()
        }
    }

    // pub fn random_unit_vector() -> Option<Self> {
    //     // Try 100 times to get a random unit vector
    //     for _ in 0..1000 {
    //         let p = Vec3::random_interval(Interval::new(-1.0, 1.0));
    //         let lensq = p.length_squared();
    //         // If the length is between 0 and 1, return the unit vector
    //         // Prevent p from being the zero vector
    //         if lensq < 1.0 && 1e-160 < lensq {
    //             return Some(p.unit_vector());
    //         }
    //     }
    //     eprintln!("Vec3::random_unit_vector() failed to find a unit vector");
    //     None
    // }

    // pub fn random_unit_vector_on_hemisphere(normal: &Vec3) -> Self {
    //     let on_unit_sphere = Vec3::random_unit_vector();
    //     match on_unit_sphere {
    //         Some(v) => {
    //             if v.dot(normal) > 0.0 {
    //                 v
    //             } else {
    //                 0. - v
    //             }
    //         }
    //         None => normal.unit_vector(),
    //     }
    //     // if on_unit_sphere.dot(normal) > 0.0 {
    //     //     on_unit_sphere
    //     // } else {
    //     //     0. - on_unit_sphere
    //     // }
    // }
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
