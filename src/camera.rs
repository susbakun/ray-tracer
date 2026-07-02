use std::{f64::INFINITY, fmt::Write, sync::Arc};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::prelude::*;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    prelude::{Cross, random_in_unit_disk, random_number01},
    ray::Ray,
    vector::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
    pub vfov: f64,
    pub vup: Vec3,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background_color: Color,
    image_height: u64,
    center: Point3,
    pixel00_lc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn initilize(&mut self) {
        // image
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u64;

        self.center = self.lookfrom;
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        // viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_lc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * ((self.defocus_angle / 2.0).to_radians()).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &HittableList) -> Result<()> {
        self.initilize();

        // progressbar
        let total = self.image_height;
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        let pb = Arc::new(pb);

        // render
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let mut image_buffer =
            vec![Color::default(); (self.image_height * self.image_width) as usize];

        image_buffer
            .par_chunks_mut(self.image_width as usize)
            .enumerate()
            .for_each(|(j, row)| {
                pb.inc(1);

                let mut rng = rand::rng();

                for (i, col) in row.iter_mut().enumerate() {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i as f64, j as f64, &mut rng);
                        let color = self.ray_color(&ray, self.max_depth, world, &mut rng);
                        pixel_color += color;
                    }
                    pixel_color *= self.pixel_samples_scale;

                    *col = pixel_color;
                }
            });

        pb.finish_with_message("Done!");

        // writing to output
        for pixel in image_buffer.iter() {
            write_color(&mut std::io::stdout(), &pixel)?;
        }

        Ok(())
    }

    fn get_ray(&self, i: f64, j: f64, rng: &mut ThreadRng) -> Ray {
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_lc
            + (self.pixel_delta_u * (offset.x() + i))
            + (self.pixel_delta_v * (offset.y() + j));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };

        let dir = pixel_sample - ray_origin;
        let ray_time = random_number01(rng);

        Ray::new_with_time(ray_origin, dir, ray_time)
    }

    fn ray_color(&self, ray: &Ray, depth: u64, world: &HittableList, rng: &mut ThreadRng) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        let mut interval = Interval::new(0.001, INFINITY);

        if !world.hit(ray, &mut interval, &mut rec) {
            return self.background_color;
        }

        // biased towards the normal
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emission = rec.material.emitted(rec.u, rec.v, rec.p);

        if !rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered, rng)
        {
            return color_from_emission;
        }

        let color_from_scatter = self.ray_color(&scattered, depth - 1, world, rng) * attenuation;
        color_from_scatter + color_from_emission
    }

    fn sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(random_number01(rng) - 0.5, random_number01(rng) - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Vec3 {
        let p = random_in_unit_disk(rng);

        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }
}
