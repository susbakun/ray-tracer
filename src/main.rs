#![allow(dead_code)]

use anyhow::Result;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod matrix;
mod mesh;
mod model;
mod perlin;
mod prelude;
mod ray;
mod rtw_image;
mod scenes;
mod shape;
mod texture;
mod transformation;
mod vector;

use scenes::*;

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
