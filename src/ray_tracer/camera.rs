use crate::math::{
    constants::INFINITY,
    interval::Interval,
    max::max_u32,
    random::random_f64,
    vec3::{Color, Point3, Vec3},
};
use indicatif::ProgressBar;
use std::io::{self, Write};
use std::{io::stdout, sync::Arc};

use super::{
    hittable::{HitRecord, Hittable},
    materials::lambertian::Lambertian,
    ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
}

impl Camera {
    pub fn render(&mut self, world: &(dyn Hittable)) {
        self.initialize();

        let bar = ProgressBar::new((self.image_height as u64) * (self.image_width as u64));

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, world);
                }
                let _ = write_color(&mut stdout(), self.pixel_samples_scale * pixel_color);
            }
        }
        bar.finish();
    }

    pub fn new() -> Self {
        Camera {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 10,
            samples_per_pixel: 10,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 1.0 / 10.,
            max_depth: 10,
        }
    }

    pub fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = max_u32(1, (self.image_width as f64 / self.aspect_ratio) as u32);

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = Point3::new(0., 0., 0.);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    // Construct a camera ray originating from the origin and directed at randomly sampled
    // point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.y()) * self.pixel_delta_u)
            + (((j as f64) + offset.x()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray {
            orig: ray_origin,
            dir: ray_direction,
        }
    }

    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &(dyn Hittable)) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec: HitRecord = HitRecord {
            t: 0.,
            p: Point3::zero(),
            normal: Vec3::zero(),
            front_face: false,
            mat: Arc::new(Lambertian {
                albedo: Color::zero(),
            }),
        };

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut ray_scattered = Ray {
                orig: rec.p,
                dir: rec.normal + Vec3::random_unit_vector(),
            };
            let mut attenuation = Color::zero();
            if rec
                .mat
                .scatter(r, &rec, &mut attenuation, &mut ray_scattered)
            {
                return attenuation * self.ray_color(&ray_scattered, depth - 1, world);
            }
            return Color::zero();
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}

const INTENSITY: Interval = Interval { min: 0.0, max: 1.0 };

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma_corrected(r);
    g = linear_to_gamma_corrected(g);
    b = linear_to_gamma_corrected(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (256. * INTENSITY.clamp(r)) as i32;
    let gbyte = (256. * INTENSITY.clamp(g)) as i32;
    let bbyte = (256. * INTENSITY.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}

pub fn linear_to_gamma_corrected(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    return 0.;
}
