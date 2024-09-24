use crate::{
    math::vec3::Color,
    ray_tracer::{hittable::hittable::HitRecord, ray::Ray},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}
