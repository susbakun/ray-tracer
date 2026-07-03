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
    pub image_width: u64,
    image_height: u64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u64,
    sqrt_spp: usize,     // Square root of number of samples per pixel
    recip_sqrt_spp: f64, // 1 / sqrt_spp
    pub max_depth: u64,
    pub vfov: f64,
    pub vup: Vec3,
    pub lookfrom: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub lookat: Point3,
    center: Point3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pixel00_lc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    pub background_color: Color,
}

impl Camera {
    fn initilize(&mut self) {
        // image
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u64;

        self.center = self.lookfrom;

        let sqrt_spp = (self.samples_per_pixel as f64).sqrt();
        self.pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp);
        self.recip_sqrt_spp = 1.0 / sqrt_spp;
        self.sqrt_spp = sqrt_spp as usize;

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
                    for s_j in 0..self.sqrt_spp {
                        for s_i in 0..self.sqrt_spp {
                            let ray =
                                self.get_ray(i as f64, j as f64, s_i as f64, s_j as f64, &mut rng);
                            let color = self.ray_color(&ray, self.max_depth, world, &mut rng);
                            pixel_color += color;
                        }
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

    fn get_ray(&self, i: f64, j: f64, s_i: f64, s_j: f64, rng: &mut ThreadRng) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j for stratified sample square s_i, s_j.

        let offset = self.sample_square_stratified(s_i, s_j, rng);
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

    fn sample_square_stratified(&self, s_i: f64, s_j: f64, rng: &mut ThreadRng) -> Vec3 {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].

        let px = ((s_i + random_number01(rng)) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j + random_number01(rng)) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }

    fn sample_square(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(random_number01(rng) - 0.5, random_number01(rng) - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Vec3 {
        let p = random_in_unit_disk(rng);

        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }
}
