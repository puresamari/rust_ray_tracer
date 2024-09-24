use serde::{Deserialize, Serialize};

use crate::{
    math::vec3::{Color, Vec3},
    ray_tracer::{hittable::hittable::HitRecord, ray::Ray},
};

use super::material::Material;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut reflected = r_in.direction().reflect(&hit_record.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *ray_scattered = Ray::new_with_time(hit_record.p, reflected, r_in.time());
        *attenuation = self.albedo;
        return ray_scattered.direction().dot(&hit_record.normal) > 0.;
    }
}
