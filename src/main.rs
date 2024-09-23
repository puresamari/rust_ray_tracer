mod math;
mod ray_tracer;

use std::sync::Arc;

use math::interval::Interval;
use math::vec3::{Color, Point3};
use ray_tracer::camera::CameraConfig;
use ray_tracer::materials::dialectric::Dialectric;
use ray_tracer::materials::lambertian::Lambertian;
use ray_tracer::materials::metal::Metal;
use ray_tracer::primitives::sphere::Sphere;
use ray_tracer::{camera::Camera, hittable_list::HittableList};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = math::random::random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * math::random::random_f64(),
                0.2,
                b as f64 + 0.9 * math::random::random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // let sphere_material: Arc<dyn ray_tracer::materials::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian { albedo });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_interval(Interval::new(0.5, 1.0));
                    let fuzz = Interval::new(0.0, 0.5).random();
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let glass_outer_mat = Arc::new(Dialectric {
                        refraction_index: 1.5,
                    });
                    let glass_inner_mat = Arc::new(Dialectric {
                        refraction_index: 1. / glass_outer_mat.refraction_index,
                    });

                    world.add(Arc::new(Sphere::new(center, 0.2, glass_outer_mat)));
                    world.add(Arc::new(Sphere::new(center, 0.1, glass_inner_mat)));
                }
            }
        }
    }

    let material1 = Arc::new(Dialectric {
        refraction_index: 1.5,
    });
    let material1_inner = Arc::new(Dialectric {
        refraction_index: 1. / material1.refraction_index,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material1_inner,
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::new_with_config(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov_in_degrees: 20.0,
        lookfrom: Point3::new(13., 2., 3.),
        lookat: Point3::new(0., 0., 0.),
        vup: Point3::new(0.0, 1.0, 0.0),

        defocus_angle_in_degrees: 0.6,
        focus_dist: 10.,
    });

    camera.render(&world);
}
