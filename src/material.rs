use rand::rngs::ThreadRng;

use crate::{
    color::Color,
    hittable::HitRecord,
    prelude::{Dot, random_unit_vector},
    ray::Ray,
};

pub trait Material {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _rng: &mut ThreadRng,
    ) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut reflected = ray_in.dir().reflect(&rec.normal);
        reflected = reflected.unit_vector() + (random_unit_vector(rng) * self.fuzz);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.dir().dot(rec.normal) > 0.0
    }
}
