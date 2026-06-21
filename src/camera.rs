use std::f64::INFINITY;

use anyhow::Result;
use indicatif::ProgressBar;
use rand::prelude::*;

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    prelude::{random_number01, random_unit_vector},
    ray::Ray,
    vector::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
    image_height: u64,
    center: Point3,
    pixel00_lc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    rng: ThreadRng,
}

impl Camera {
    fn initilize(&mut self) {
        // random generator
        self.rng = rand::rng();

        // image
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u64;

        self.center = Point3::new(0.0, 0.0, 0.0);
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);
                    let color = self.ray_color(&ray, self.max_depth, world);
                    pixel_color += color;
                }
                pixel_color *= self.pixel_samples_scale;

                write_color(&mut std::io::stdout(), &pixel_color)?;
            }
        }
        pb.finish_with_message("Done!");

        Ok(())
    }

    fn get_ray(&mut self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_lc
            + (self.pixel_delta_u * (offset.x() + i))
            + (self.pixel_delta_v * (offset.y() + j));

        let ray_origin = self.center;
        let dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, dir)
    }

    fn ray_color(&mut self, ray: &Ray, depth: u64, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        let interval = Interval::from(0.001, INFINITY);

        if world.hit(ray, interval, &mut rec) {
            // biased towards the normal
            let dir = rec.normal + random_unit_vector(&mut self.rng);
            return self.ray_color(&Ray::new(rec.p, dir), depth - 1, world) * 0.5;
        }

        let unit_dirction = ray.dir().unit_vector();
        let a = 0.5 * (unit_dirction.y() + 1.0);
        (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
    }

    fn sample_square(&mut self) -> Vec3 {
        Vec3::new(
            random_number01(&mut self.rng) - 0.5,
            random_number01(&mut self.rng) - 0.5,
            0.0,
        )
    }
}
