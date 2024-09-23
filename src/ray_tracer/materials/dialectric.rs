use crate::{
    math::vec3::Color,
    ray_tracer::{hittable::HitRecord, material::Material, ray::Ray},
};

pub struct Dialectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dialectric {
    fn reflactance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Dialectric::reflactance(cos_theta, ri) > rand::random() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };

        // let refracted = unit_direction.refract(&hit_record.normal, ri);

        *ray_scattered = Ray {
            orig: hit_record.p,
            dir: direction,
        };
        return true;
    }
}
