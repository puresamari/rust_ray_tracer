use math::interval::Interval;
use serde::{Deserialize, Serialize};

use crate::ray::Ray;

use super::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    primitives::sphere::Sphere,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HittableObject {
    Sphere(Sphere),
    List(HittableList),
}

impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            HittableObject::List(list) => list.hit(r, ray_t, rec),
        }
    }
}
