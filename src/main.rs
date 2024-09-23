mod math;
mod ray_tracer;

use std::sync::Arc;

use math::vec3::{Color, Point3};
use ray_tracer::camera::CameraConfig;
use ray_tracer::materials::dialectric::Dialectric;
use ray_tracer::materials::lambertian::Lambertian;
use ray_tracer::materials::metal::Metal;
use ray_tracer::primitives::sphere::Sphere;
use ray_tracer::{camera::Camera, hittable_list::HittableList};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dialectric {
        refraction_index: 1.5,
    });
    let material_bubble = Arc::new(Dialectric {
        refraction_index: 1. / 1.5,
    });
    let material_right = Arc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new_with_config(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov_in_degrees: 90.0,
    });

    camera.render(&world);
}
