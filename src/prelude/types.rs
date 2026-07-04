use std::sync::Arc;

use crate::{hittable::Hittable, material::Material, texture::Texture};

pub type MaterialType = Arc<dyn Material + Send + Sync>;
pub type TextureType = Arc<dyn Texture + Send + Sync>;
pub type HittableType = Arc<dyn Hittable + Send + Sync>;

pub struct UV {
    pub u: f64,
    pub v: f64,
}

impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}
