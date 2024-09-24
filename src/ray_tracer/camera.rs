use crate::math::{
    circle::degrees_to_radians,
    constants::INFINITY,
    interval::Interval,
    max::max_u32,
    random::random_f64,
    vec3::{Color, Point3, Vec3},
};
use image::Rgb;
use indicatif::ProgressBar;

extern crate image;

use super::{
    hittable::hittable::{HitRecord, Hittable},
    material::{lambertian::Lambertian, material::Material, object::MaterialObject},
    ray::Ray,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub vfov_in_degrees: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    /// Variation angle of rays through each pixel
    pub defocus_angle_in_degrees: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,
}

pub struct Camera {
    pub config: CameraConfig,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples

    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &(dyn Hittable)) {
        self.initialize();

        let bar =
            ProgressBar::new((self.image_height as u64) * (self.config.image_width as u64) + 1);

        let mut imgbuf = image::ImageBuffer::new(self.config.image_width, self.image_height);

        for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut pixel_color = Color::zero();
            for _ in 0..self.config.samples_per_pixel {
                let ray = self.get_ray(i, j);
                pixel_color = pixel_color + self.ray_color(&ray, self.config.max_depth, world);
            }
            *pixel = (self.pixel_samples_scale * pixel_color).to_pixel();
            bar.inc(1);
        }

        imgbuf.save("image.png").unwrap();
        bar.inc(1);
        bar.finish();
    }

    pub fn new_with_config(config: CameraConfig) -> Self {
        Camera {
            config,
            image_height: 10,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 1.0 / 10.,
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }

    pub fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = max_u32(
            1,
            (self.config.image_width as f64 / self.config.aspect_ratio) as u32,
        );

        self.pixel_samples_scale = 1.0 / (self.config.samples_per_pixel as f64);

        self.center = self.config.lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.config.vfov_in_degrees);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2. * h * self.config.focus_dist;
        let viewport_width =
            viewport_height * ((self.config.image_width as f64) / (self.image_height as f64));

        self.w = (self.config.lookfrom - self.config.lookat).unit_vector();
        self.u = self.config.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * self.v.inverted();

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.config.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - (self.config.focus_dist * self.w) - (viewport_u / 2.) - (viewport_v / 2.);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.config.focus_dist
            * f64::tan(degrees_to_radians(
                self.config.defocus_angle_in_degrees / 2.,
            ));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly
    // sampled point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.config.defocus_angle_in_degrees <= 0. {
            self.center
        } else {
            self.defocus_disc_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disc_sample(&self) -> Vec3 {
        // TODO: This can be a vec2
        let p = Vec3::random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
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
            material: MaterialObject::Lambertian(Lambertian {
                albedo: Color::zero(),
            }),
        };

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut ray_scattered = Ray::new(rec.p, rec.normal + Vec3::random_unit_vector());
            let mut attenuation = Color::zero();
            if rec
                .material
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

impl Color {
    pub fn to_pixel(&self) -> Rgb<u8> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Apply a linear to gamma transform for gamma 2
        r = linear_to_gamma_corrected(r);
        g = linear_to_gamma_corrected(g);
        b = linear_to_gamma_corrected(b);

        // Translate the [0,1] component values to the byte range [0,255].
        let rbyte = (256. * INTENSITY.clamp(r)) as u8;
        let gbyte = (256. * INTENSITY.clamp(g)) as u8;
        let bbyte = (256. * INTENSITY.clamp(b)) as u8;

        return Rgb([rbyte, gbyte, bbyte]);
    }
}

pub fn linear_to_gamma_corrected(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    return 0.;
}
