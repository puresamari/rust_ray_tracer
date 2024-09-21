mod math;
mod ray_tracer;

use std::{io, sync::Arc};

use math::{
    constants::INFINITY,
    max::max_i32,
    vec3::{write_color, Color, Point3, Vec3},
};
use ray_tracer::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    primitives::sphere::Sphere,
    ray::Ray,
};

fn ray_color(r: &Ray, world: &(dyn Hittable)) -> Color {
    let mut rec: HitRecord = HitRecord {
        t: 0.,
        p: Point3::zero(),
        normal: Vec3::zero(),
        front_face: false,
    };

    if world.hit(r, 0., INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::one());
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // World

    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = max_i32(1, (image_width as f64 / aspect_ratio) as i32);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + ((i as f64) * pixel_delta_u) + ((j as f64) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);

            let _ = write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("Done.\n");
}
