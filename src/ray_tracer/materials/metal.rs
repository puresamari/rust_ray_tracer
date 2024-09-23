use crate::{
    math::vec3::Color,
    ray_tracer::{hittable::HitRecord, material::Material, ray::Ray},
};

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().reflect(&hit_record.normal);
        *ray_scattered = Ray {
            orig: hit_record.p,
            dir: reflected,
        };
        *attenuation = self.albedo;
        true
    }
}
