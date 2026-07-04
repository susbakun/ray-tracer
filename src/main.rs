#![allow(dead_code)]
use std::sync::Arc;

use anyhow::Result;
use rand::rng;

use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    constant_medium::ConstantMedium,
    hittable::{RotateY, Translate},
    hittable_list::HittableList,
    interval::Interval,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    model::Model,
    prelude::*,
    quad::{Quad, create_box},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    triangle::Triangle,
    vector::{Point3, Vec3},
};

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod matrix;
mod mesh;
mod model;
mod perlin;
mod prelude;
mod quad;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod triangle;
mod vector;

fn bouncing_sphere() -> Result<()> {
    let mut rng = rng();

    // world
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new_stationary(
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center
                        + Point3::new(
                            0.0,
                            random_number_range(&mut rng, Interval::new(0.0, 0.5)),
                            0.0,
                        );
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vector_range(&mut rng, Interval::new(0.5, 1.0));
                    let fuzz = random_number_range(&mut rng, Interval::new(0.0, 0.5));
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh = Arc::new(BVH::bvh_node(&mut world, &mut rng));
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

    // sphere
    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from(checker.clone())),
    )));

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

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn earth() -> Result<()> {
    let mut world = HittableList::new();

    // earth
    let earth_texture = ImageTexture::new("earthmap.jpg")?;
    let earth_surface = Arc::new(Lambertian::from(Arc::new(earth_texture)));
    let globe = Sphere::new_stationary(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    world.add(Arc::new(globe));

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

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn perlin_spheres() -> Result<()> {
    let mut world = HittableList::new();

    // perlin sphere
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let lamper = Arc::new(Lambertian::from(pertext));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lamper.clone(),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        lamper.clone(),
    )));

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

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;

    camera.render(&world)?;

    Ok(())
}

fn quads() -> Result<()> {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    // camera
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
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let lamper = Arc::new(Lambertian::from(pertext));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lamper.clone(),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        lamper.clone(),
    )));

    // light
    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    // camera
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
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    // cornell box sides
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // boxes
    let mut box1: HittableType = Arc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: HittableType = Arc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    // camera
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

fn cornell_smoke() -> Result<()> {
    let mut world = HittableList::new();

    // Materials
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    // cornell box sides
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // boxes
    let mut box1: HittableType = Arc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: HittableType = Arc::new(create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    // camera
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

fn final_scene(image_width: u64, samples_per_pixel: u64, max_depth: u64) -> Result<()> {
    let mut boxes1 = HittableList::new();

    let mut rng = rand::rng();

    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + w * (i as f64);
            let z0 = -1000.0 + w * (j as f64);
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = random_number_range(&mut rng, Interval::new(1.0, 101.0));

            boxes1.add(Arc::new(create_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BVH::bvh_node(&mut boxes1, &mut rng)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 400.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material.clone(),
    )));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundry = Arc::new(Sphere::new_stationary(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundry.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundry.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundry = Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundry,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_texture = ImageTexture::new("earthmap.jpg")?;
    let emat = Arc::new(Lambertian::from(Arc::new(earth_texture)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new_stationary(
            random_vector_range(&mut rng, Interval::new(0.0, 165.0)),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVH::bvh_node(&mut boxes2, &mut rng)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel;
    camera.max_depth = max_depth;
    camera.background_color = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(478.0, 278.0, -600.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 20.0;

    camera.render(&world)?;

    Ok(())
}

fn triangle_scene() -> Result<()> {
    let mut world = HittableList::new();

    let triangle_mat = Arc::new(Lambertian::new(Color::new(0.9, 0.2, 0.1)));

    world.add(Arc::new(Triangle::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        triangle_mat,
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 20.0;

    camera.render(&world)?;

    Ok(())
}

fn teapot_scene() -> Result<()> {
    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Color::new(0.45, 0.45, 0.45)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.5, 0.0),
        1000.0,
        ground,
    )));

    let model = Model::new("utah_teapot.obj")?;
    let model = Arc::new(model);

    world.add(model);

    let light = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 4.0, -2.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        light,
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 400;
    camera.max_depth = 50;
    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 2.0, 5.0);
    camera.lookat = Point3::new(0.0, 1.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 5.0;

    camera.render(&world)?;

    Ok(())
}

fn main() -> Result<()> {
    let scene = 11;
    match scene {
        1 => bouncing_sphere(),
        2 => checked_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        10 => triangle_scene(),
        11 => teapot_scene(),
        _ => final_scene(400, 250, 4),
    }
}
