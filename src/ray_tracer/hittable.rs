use std::sync::Arc;

use crate::math::{interval::Interval, vec3::Vec3};

use super::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub mat: Arc<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outwawrd_normal` is assumed to have unit length.
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            0. - outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
