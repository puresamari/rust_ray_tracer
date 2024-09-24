use crate::{
    math::random,
    ray_tracer::{
        animation::{AnimatedValue, AnimatedVec3, AnimationContext},
        camera::{Camera, CameraConfig},
        hittable::hittable_list::HittableList,
    },
};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};
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

pub enum RenderType {
    SingleFrame(u32),
    Animation(u32, u32),
}

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

    /// Output path, scene path + output
    pub fn output_path(&self) -> String {
        self.directory.clone() + "/output"
    }

    pub fn render_frame(&mut self, frame: u32) {
        let world_arc = Arc::new(self.world.clone());

        let rendered_frame = self.camera.render_frame(world_arc, frame);

        // Create the image buffer
        let mut imgbuf =
            image::ImageBuffer::new(self.camera.config.image_width, self.camera.image_height());

        // Iterate through the rendered pixel data and assign it to the image buffer
        for (i, pixel) in rendered_frame.iter().enumerate() {
            let x = (i % self.camera.config.image_width as usize) as u32;
            let y = (i / self.camera.config.image_width as usize) as u32;
            let pixel = image::Rgb(*pixel); // Convert the pixel to the expected Rgb format
            imgbuf.put_pixel(x, y, pixel);
        }

        // Ensure the path exists
        if !std::path::Path::new(&self.output_path()).exists() {
            std::fs::create_dir_all(&self.output_path()).unwrap();
        }

        let image_path = format!("{}/frame-{}.png", self.output_path(), frame);

        imgbuf.save(image_path).unwrap();
    }

    pub fn render_animation(&mut self, start_frame: u32, frames: u32) {
        (start_frame..(start_frame + frames))
            .into_iter()
            .for_each(|frame| {
                self.render_frame(frame);
            });
    }

    pub fn render(&mut self, render_type: RenderType) {
        match render_type {
            RenderType::SingleFrame(frame) => self.render_frame(frame),
            RenderType::Animation(start_frame, frames) => {
                self.render_animation(start_frame, frames)
            }
        }
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

                animation_meta: AnimationContext {
                    frames_per_second: 24,
                    shutter_speed: 1. / 200.,
                },

                defocus_angle_in_degrees: 0.6,
                focus_dist: 10.,
            },
            directory,
        );

        let ground_material = MaterialObject::Lambertian(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            AnimatedVec3::static_value(Point3::new(0.0, -1000.0, 0.0)),
            1000.0,
            ground_material,
        )));

        fn create_animated_vec3(
            center_point: Point3,
            frequency_interval: Interval,
            amplitude_interval: Interval,
            phase_shift_interval: Interval,
        ) -> AnimatedVec3 {
            AnimatedVec3 {
                x: AnimatedValue::Sinusoidal {
                    baseline: center_point.x(),
                    frequency: frequency_interval.random(),
                    amplitude: amplitude_interval.random(),
                    phase_shift: phase_shift_interval.random(),
                },
                y: AnimatedValue::Sinusoidal {
                    baseline: center_point.y(),
                    frequency: frequency_interval.random(),
                    amplitude: amplitude_interval.random(),
                    phase_shift: phase_shift_interval.random(),
                },
                z: AnimatedValue::Sinusoidal {
                    baseline: center_point.z(),
                    frequency: frequency_interval.random(),
                    amplitude: amplitude_interval.random(),
                    phase_shift: phase_shift_interval.random(),
                },
            }
        }

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_f64();
                let center_point = Point3::new(
                    a as f64 + 0.9 * random_f64(),
                    0.2,
                    b as f64 + 0.9 * random_f64(),
                );

                let center = if random_f64() < 0.8 {
                    let frequency_interval = Interval::new(1.0, 4.0);
                    let amplitude_interval = Interval::new(0.1, 0.3);
                    let phase_shift_interval = Interval::new(0.0, 2.0 * std::f64::consts::PI);
                    create_animated_vec3(
                        center_point,
                        frequency_interval,
                        amplitude_interval,
                        phase_shift_interval,
                    )
                } else {
                    AnimatedVec3::static_value(center_point)
                };

                if (center_point - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    // let sphere_material: Arc<dyn ray_tracer::materials::Material>;
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = MaterialObject::Lambertian(Lambertian { albedo });
                        scene.world.add(HittableObject::Sphere(Sphere::new(
                            center,
                            0.2,
                            sphere_material,
                        )));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_interval(Interval::new(0.5, 1.0));
                        let fuzz = Interval::new(0.0, 0.5).random();
                        let sphere_material = MaterialObject::Metal(Metal { albedo, fuzz });
                        scene.world.add(HittableObject::Sphere(Sphere::new(
                            center,
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

                        scene.world.add(HittableObject::Sphere(Sphere::new(
                            center,
                            0.2,
                            glass_outer_mat,
                        )));
                        scene.world.add(HittableObject::Sphere(Sphere::new(
                            center,
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
            AnimatedVec3::static_value(Point3::new(0.0, 1.0, 0.0)),
            1.0,
            material1,
        )));
        scene.world.add(HittableObject::Sphere(Sphere::new(
            AnimatedVec3::static_value(Point3::new(0.0, 1.0, 0.0)),
            0.9,
            material1_inner,
        )));

        let material2 = MaterialObject::Lambertian(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            create_animated_vec3(
                Point3::new(-4.0, 1.0, 0.0),
                Interval::new(1.0, 4.0),
                Interval::new(0.1, 0.3),
                Interval::new(0.0, 2.0 * std::f64::consts::PI),
            ),
            1.0,
            material2,
        )));

        let material3 = MaterialObject::Metal(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        });
        scene.world.add(HittableObject::Sphere(Sphere::new(
            create_animated_vec3(
                Point3::new(4.0, 1.0, 0.0),
                Interval::new(1.0, 1.0),
                Interval::new(0.1, 0.15),
                Interval::new(0.0, 2.0 * std::f64::consts::PI),
            ),
            1.0,
            material3,
        )));

        return scene;
    }
}
