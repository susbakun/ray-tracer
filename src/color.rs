use anyhow::Result;
use std::io::Stdout;
use std::io::Write;

use crate::interval::Interval;
use crate::vector::Vec3;

pub type Color = Vec3;

pub fn write_color(output: &mut Stdout, pixel_color: &Color) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::from(0.0, 0.999);

    // apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let ir = (255.999 * intensity.clamp(r)) as u16;
    let ig = (255.999 * intensity.clamp(g)) as u16;
    let ib = (255.999 * intensity.clamp(b)) as u16;

    writeln![output, "{ir} {ig} {ib}"]?;

    Ok(())
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.powf(1.0 / 2.2);
    }

    return 0.0;
}
