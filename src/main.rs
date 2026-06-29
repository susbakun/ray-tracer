#![allow(dead_code)]
use std::{panic::PanicHookInfo, rc::Rc};

use anyhow::Result;
use rand::rng;

use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    hittable::{Hittable, RotateY, Translate},
    hittable_list::HittableList,
    interval::Interval,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    prelude::{random_number_range, random_number01, random_vector_range, random_vector01},
    quad::{Quad, create_box},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    vector::{Point3, Vec3},
};

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod prelude;
mod quad;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod vector;

fn bouncing_sphere() -> Result<()> {
    let mut rng = rng();

    // world
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new_stationary(
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
                    let center2 = center
                        + Point3::new(
                            0.0,
                            random_number_range(&mut rng, Interval::new(0.0, 0.5)),
                            0.0,
                        );
                    world.add(Rc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vector_range(&mut rng, Interval::new(0.5, 1.0));
                    let fuzz = random_number_range(&mut rng, Interval::new(0.0, 0.5));
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh = Rc::new(BVH::bvh_node(&mut world, &mut rng));
    world = HittableList::from(bvh);

    // camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn checked_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from(checker.clone())),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn earth() -> Result<()> {
    let mut world = HittableList::new();

    let earth_texture = ImageTexture::new("earthmap.jpg")?;
    let earth_surface = Rc::new(Lambertian::from(Rc::new(earth_texture)));
    let globe = Sphere::new_stationary(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    world.add(Rc::new(globe));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn perlin_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    let lamper = Rc::new(Lambertian::from(pertext));

    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lamper.clone(),
    )));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        lamper.clone(),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn quads() -> Result<()> {
    let mut world = HittableList::new();

    // Materials
    let left_red = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 9.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 20.0;

    camera.render(&world)?;

    Ok(())
}

fn simple_light() -> Result<()> {
    let mut world = HittableList::new();

    // Materials
    let pertext = Rc::new(NoiseTexture::new(4.0));
    let lamper = Rc::new(Lambertian::from(pertext));

    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lamper.clone(),
    )));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        lamper.clone(),
    )));

    let difflight = Rc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new_stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(26.0, 3.0, 6.0);
    camera.lookat = Point3::new(0.0, 2.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 20.0;

    camera.render(&world)?;

    Ok(())
}

fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();

    // Materials
    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // boxes
    let mut box1: Rc<dyn Hittable> = Rc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = Rc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 20.0;

    camera.render(&world)?;

    Ok(())
}

fn main() -> Result<()> {
    let scene = 7;
    match scene {
        1 => bouncing_sphere(),
        2 => checked_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => unreachable!(),
    }
}
