#![allow(dead_code)]
use std::rc::Rc;

use anyhow::Result;
use rand::rng;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    interval::Interval,
    material::{Dielectric, Lambertian, Metal},
    prelude::{random_number_range, random_number01, random_vector_range, random_vector01},
    sphere::Sphere,
    vector::{Point3, Vec3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod prelude;
mod ray;
mod sphere;
mod vector;

fn main() -> Result<()> {
    let mut rng = rng();

    // world
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_number01(&mut rng);
            let center = Point3::new(
                (a as f64) + 0.9 * random_number01(&mut rng),
                0.2,
                (b as f64) + 0.9 * random_number01(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vector01(&mut rng) * random_vector01(&mut rng);
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vector_range(&mut rng, Interval::from(0.5, 1.0));
                    let fuzz = random_number_range(&mut rng, Interval::from(0.0, 0.5));
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}
