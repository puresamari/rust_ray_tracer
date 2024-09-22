use std::io::stdout;

use crate::math::{
    constants::INFINITY,
    interval::Interval,
    max::max_i32,
    vec3::{write_color, Color, Point3, Vec3},
};

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &(dyn Hittable)) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + ((i as f64) * self.pixel_delta_u)
                    + ((j as f64) * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);

                let _ = write_color(&mut stdout(), pixel_color);
            }
        }
        eprint!("Done.\n");
    }

    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    pub fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = max_i32(1, (self.image_width as f64 / self.aspect_ratio) as i32);

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

    fn ray_color(&self, r: &Ray, world: &(dyn Hittable)) -> Color {
        let mut rec: HitRecord = HitRecord {
            t: 0.,
            p: Point3::zero(),
            normal: Vec3::zero(),
            front_face: false,
        };

        if world.hit(r, Interval::new(0., INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::one());
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}
