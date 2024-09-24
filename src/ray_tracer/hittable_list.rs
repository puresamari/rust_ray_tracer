use std::sync::Arc;

use crate::math::interval::Interval;

use super::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray_tracer::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::ray_tracer::hittable::HitRecord,
    ) -> bool {
        let temp_rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                match &temp_rec {
                    Some(temp_rec) => {
                        rec.t = temp_rec.t;
                        rec.p = temp_rec.p;
                        rec.normal = temp_rec.normal;
                        rec.front_face = temp_rec.front_face;
                    }
                    None => {}
                }
            }
        }

        return hit_anything;
    }
}
