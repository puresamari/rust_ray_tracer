use crate::{
    math::vec3::{Color, Vec3},
    ray_tracer::{hittable::HitRecord, material::Material, ray::Ray},
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *ray_scattered = Ray::new_with_time(hit_record.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;

        true
    }
}
