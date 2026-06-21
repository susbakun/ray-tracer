#![allow(dead_code)]
use anyhow::Result;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vector::Point3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod prelude;
mod ray;
mod sphere;
mod vector;

fn main() -> Result<()> {
    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world)?;

    Ok(())
}
