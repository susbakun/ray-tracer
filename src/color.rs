use anyhow::Result;
use std::io::Stdout;
use std::io::Write;

use crate::vector::Vec3;

pub type Color = Vec3;

pub fn write_color(output: &mut Stdout, pixel_color: &Color) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let ir = (255.999 * r) as u16;
    let ig = (255.999 * g) as u16;
    let ib = (255.999 * b) as u16;

    writeln![output, "{ir} {ig} {ib}"]?;

    Ok(())
}
