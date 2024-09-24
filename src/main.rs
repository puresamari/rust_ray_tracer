mod math;
mod ray_tracer;
mod scene;

use std::sync::Arc;

use math::interval::Interval;
use math::random::random_f64;
use math::vec3::{Color, Point3};
use ray_tracer::camera::CameraConfig;
use ray_tracer::hittable_list::HittableList;
use ray_tracer::materials::dialectric::Dialectric;
use ray_tracer::materials::lambertian::Lambertian;
use ray_tracer::materials::metal::Metal;
use ray_tracer::primitives::sphere::Sphere;
use scene::scene::Scene;

fn create_example_scene() -> Scene {
    let mut scene = Scene::new(
        HittableList::new(),
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 32,
            max_depth: 50,

            vfov_in_degrees: 20.0,
            lookfrom: Point3::new(13., 2., 3.),
            lookat: Point3::new(0., 0., 0.),
            vup: Point3::new(0.0, 1.0, 0.0),

            defocus_angle_in_degrees: 0.6,
            focus_dist: 10.,
        },
    );

    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    scene.world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            let mut center1 = center;
            if random_f64() < 0.7 {
                let max_movement = Interval::new(0.0, 0.5);
                center1 = center
                    + Point3::new(
                        max_movement.random(),
                        max_movement.random(),
                        max_movement.random(),
                    );
            }

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // let sphere_material: Arc<dyn ray_tracer::materials::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian { albedo });
                    scene.world.add(Arc::new(Sphere::new_with_movement(
                        center,
                        center1,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_interval(Interval::new(0.5, 1.0));
                    let fuzz = Interval::new(0.0, 0.5).random();
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    scene.world.add(Arc::new(Sphere::new_with_movement(
                        center,
                        center1,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let glass_outer_mat = Arc::new(Dialectric {
                        refraction_index: 1.5,
                    });
                    let glass_inner_mat = Arc::new(Dialectric {
                        refraction_index: 1. / glass_outer_mat.refraction_index,
                    });

                    scene.world.add(Arc::new(Sphere::new_with_movement(
                        center,
                        center1,
                        0.2,
                        glass_outer_mat,
                    )));
                    scene.world.add(Arc::new(Sphere::new_with_movement(
                        center,
                        center1,
                        0.1,
                        glass_inner_mat,
                    )));
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
    scene.world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    scene.world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material1_inner,
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    scene.world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    scene.world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return scene;
}

fn main() {
    let mut scene = create_example_scene();
    scene.render();
}
