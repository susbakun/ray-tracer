use std::f64::INFINITY;

use anyhow::Result;
use indicatif::ProgressBar;

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vector::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    image_height: u64,
    center: Point3,
    pixel00_lc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    fn initilize(&mut self) {
        // image
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u64;

        self.center = Point3::new(0.0, 0.0, 0.0);

        // viewport
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_lc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn render(&mut self, world: &HittableList) -> Result<()> {
        self.initilize();

        // progressbar
        let total = self.image_height;
        let pb = ProgressBar::new(total);

        // render
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        pb.println("Progress:");
        for j in 0..self.image_height {
            pb.inc(1);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_lc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&ray, &world);
                write_color(&mut std::io::stdout(), &pixel_color)?;
            }
        }
        pb.finish_with_message("Done!");

        Ok(())
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();

        let interval = Interval::from(0.0, INFINITY);

        if world.hit(ray, interval, &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_dirction = ray.dir().unit_vector();
        let a = 0.5 * (unit_dirction.y() + 1.0);
        (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
    }
}
