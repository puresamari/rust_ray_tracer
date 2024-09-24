use serde::{Deserialize, Serialize};

use crate::{
    math::vec3::Color,
    ray_tracer::{hittable::hittable::HitRecord, ray::Ray},
};

use super::{dialectric::Dialectric, lambertian::Lambertian, material::Material, metal::Metal};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MaterialObject {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dialectric),
}

impl Material for MaterialObject {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialObject::Lambertian(lambertian) => {
                lambertian.scatter(r_in, hit_record, attenuation, ray_scattered)
            }
            MaterialObject::Metal(metal) => {
                metal.scatter(r_in, hit_record, attenuation, ray_scattered)
            }
            MaterialObject::Dielectric(dielectric) => {
                dielectric.scatter(r_in, hit_record, attenuation, ray_scattered)
            }
        }
    }
}
