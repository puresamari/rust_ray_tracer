mod math;
mod ray_tracer;

use std::sync::Arc;

use math::vec3::Point3;
use ray_tracer::{camera::Camera, hittable_list::HittableList, primitives::sphere::Sphere};

fn main() {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
