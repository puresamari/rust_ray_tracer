use crate::{
    math::vec3::{Color, Vec3},
    ray_tracer::{hittable::HitRecord, material::Material, ray::Ray},
};

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
        *ray_scattered = Ray {
            orig: hit_record.p,
            dir: reflected,
        };
        *attenuation = self.albedo;
        return ray_scattered.direction().dot(&hit_record.normal) > 0.;
    }
}
