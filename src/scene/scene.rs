use crate::ray_tracer::{
    camera::{Camera, CameraConfig},
    hittable::hittable_list::HittableList,
};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

use crate::math::interval::Interval;
use crate::math::random::random_f64;
use crate::math::vec3::{Color, Point3};
use crate::ray_tracer::hittable::object::HittableObject;
use crate::ray_tracer::hittable::primitives::sphere::Sphere;
use crate::ray_tracer::material::dialectric::Dialectric;
use crate::ray_tracer::material::lambertian::Lambertian;
use crate::ray_tracer::material::metal::Metal;
use crate::ray_tracer::material::object::MaterialObject;

pub struct Scene {
    pub world: HittableList,
    camera: Camera,
    directory: String,
}

#[derive(Serialize, Deserialize)]
struct SceneConfig {
    camera: CameraConfig,
    world: HittableList,
}

pub const SCENE_FILE_EXTENSION: &str = ".rrtscene";

impl Scene {
    pub fn new(world: HittableList, config: CameraConfig, directory: String) -> Self {
        Self {
            world,
            camera: Camera::new_with_config(config),
            directory,
        }
    }

    pub fn save_config(&self, path: &str) {
        // Check if the file extension is .toml
        if !path.ends_with(SCENE_FILE_EXTENSION) {
            panic!("The file extension must be .toml");
        }

        // Delete the file if it already exists
        fs::remove_file(path).unwrap_or_default();

        let config = SceneConfig {
            camera: self.camera.config,
            world: self.world.clone(),
        };

        // create path if it doesn't exist
        let parent = std::path::Path::new(path).parent().unwrap();
        fs::create_dir_all(parent).unwrap();

        fs::write(path, toml::to_string(&config).unwrap()).unwrap();
    }

    pub fn load_config(scene_file_path: &str) -> Self {
        // Check if the file extension is .toml
        if !scene_file_path.ends_with(SCENE_FILE_EXTENSION) {
            panic!("The file extension must be .toml");
        }

        let directory = std::path::Path::new(scene_file_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if !fs::metadata(scene_file_path).is_ok() {
            println!("File not found. Creating example scene.");
            return Self::create_example_scene(directory);
        }

        let config: SceneConfig =
            toml::from_str(&fs::read_to_string(scene_file_path).unwrap()).unwrap();

        Self {
            world: config.world,
            camera: Camera::new_with_config(config.camera),
            directory,
        }
    }

    pub fn render(&mut self) {
        // Output path, scene path + output
        let output_path = self.directory.clone() + "/output";
        self.camera.render(&self.world, output_path);
    }

    fn create_example_scene(directory: String) -> Scene {
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
            directory,
        );

        let ground_material = MaterialObject::Lambertian(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
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
                        let sphere_material = MaterialObject::Lambertian(Lambertian { albedo });
                        scene
                            .world
                            .add(HittableObject::Sphere(Sphere::new_with_movement(
                                center,
                                center1,
                                0.2,
                                sphere_material,
                            )));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_interval(Interval::new(0.5, 1.0));
                        let fuzz = Interval::new(0.0, 0.5).random();
                        let sphere_material = MaterialObject::Metal(Metal { albedo, fuzz });
                        scene
                            .world
                            .add(HittableObject::Sphere(Sphere::new_with_movement(
                                center,
                                center1,
                                0.2,
                                sphere_material,
                            )));
                    } else {
                        // glass
                        let glass_outer_mat = MaterialObject::Dielectric(Dialectric {
                            refraction_index: 1.5,
                        });
                        let glass_inner_mat = MaterialObject::Dielectric(Dialectric {
                            refraction_index: 1. / 1.5,
                        });

                        scene
                            .world
                            .add(HittableObject::Sphere(Sphere::new_with_movement(
                                center,
                                center1,
                                0.2,
                                glass_outer_mat,
                            )));
                        scene
                            .world
                            .add(HittableObject::Sphere(Sphere::new_with_movement(
                                center,
                                center1,
                                0.1,
                                glass_inner_mat,
                            )));
                    }
                }
            }
        }

        let material1 = MaterialObject::Dielectric(Dialectric {
            refraction_index: 1.5,
        });
        let material1_inner = MaterialObject::Dielectric(Dialectric {
            refraction_index: 1. / 1.5,
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));
        scene.world.add(HittableObject::Sphere(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            0.9,
            material1_inner,
        )));

        let material2 = MaterialObject::Lambertian(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = MaterialObject::Metal(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        return scene;
    }
}
