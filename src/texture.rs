use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::{
    color::Color, interval::Interval, perlin::Perlin, prelude::*, rtw_image::RTWImage,
    vector::Point3,
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }

    pub fn from(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: TextureType,
    odd: TextureType,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Color, odd: Color) -> Self {
        let inv_scale = 1.0 / scale;
        let even = Arc::new(SolidColor::from(even));
        let odd = Arc::new(SolidColor::from(odd));

        Self {
            inv_scale,
            even,
            odd,
        }
    }

    pub fn from(scale: f64, even: TextureType, odd: TextureType) -> Self {
        let inv_scale = 1.0 / scale;

        Self {
            inv_scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (p.x() * self.inv_scale).floor() as isize;
        let y_integer = (p.y() * self.inv_scale).floor() as isize;
        let z_integer = (p.z() * self.inv_scale).floor() as isize;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    pub fn new(file_name: impl AsRef<Path>) -> Result<Self> {
        let image = RTWImage::new(file_name)?;

        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * (self.image.width() as f64)) as usize;
        let j = (v * (self.image.height() as f64)) as usize;

        let pixeld_data = self.image.pixel_data(i, j);

        Color::from(pixeld_data)
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        let noise = Perlin::new(256);

        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&p, 7)).sin())
    }
}
