use serde::{Deserialize, Serialize};

use crate::{
    math::{
        interval::Interval,
        max::max_f64,
        vec3::{Point3, Vec3},
    },
    ray_tracer::{
        hittable::hittable::{HitRecord, Hittable},
        material::object::MaterialObject,
        ray::Ray,
    },
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Sphere {
    center: Ray,
    radius: f64,

    material: MaterialObject,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialObject) -> Self {
        Sphere {
            center: Ray::new(center, Vec3::zero()),
            radius: max_f64(radius, 0.),
            material,
        }
    }

    pub fn new_with_movement(
        center0: Point3,
        center1: Point3,
        radius: f64,
        material: MaterialObject,
    ) -> Self {
        let center = Ray::new(center0, center1 - center0);
        Sphere {
            center,
            radius: max_f64(radius, 0.),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material;

        return true;
    }
}
