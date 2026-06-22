use rand::rngs::ThreadRng;

use crate::{
    color::Color,
    hittable::HitRecord,
    prelude::{Dot, random_number01, random_unit_vector},
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.dir().unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        if cannot_refract || Self::reflectance(cos_theta, ri) > random_number01(rng) {
            direction = unit_direction.reflect(&rec.normal);
        } else {
            direction = unit_direction.refract(&rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
