use std::{f64::INFINITY, rc::Rc};

use crate::{
    aabb::AABB,
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::{self, Interval},
    material::{Isotropic, Material},
    prelude::random_number01,
    ray::Ray,
    texture::Texture,
    vector::Vec3,
};

pub struct ConstantMedium {
    boundry: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundry: Rc<dyn Hittable>, density: f64, tex: Rc<dyn Texture>) -> Self {
        let neg_inv_density = -1.0 / density;
        let phase_function = Rc::new(Isotropic::from(tex));

        Self {
            boundry,
            neg_inv_density,
            phase_function,
        }
    }

    pub fn from_color(boundry: Rc<dyn Hittable>, density: f64, color: Color) -> Self {
        let neg_inv_density = -1.0 / density;
        let phase_function = Rc::new(Isotropic::new(color));

        Self {
            boundry,
            neg_inv_density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        let mut rng = rand::rng();

        let mut in_t = interval::UNIVERSE;

        if !self.boundry.hit(ray, &mut in_t, &mut rec1) {
            return false;
        }

        let mut exit_t = Interval::new(rec1.t + 0.001, INFINITY);
        if !self.boundry.hit(ray, &mut exit_t, &mut rec2) {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t <= 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.dir().length();
        let distance_inside_boundry = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_number01(&mut rng).ln();

        if hit_distance > distance_inside_boundry {
            return false;
        }

        rec.t = rec1.t + (hit_distance / ray_length);
        rec.p = ray.at(rec.t);

        rec.normal = Vec3::new(0.0, 1.0, 0.0); // arbitary
        rec.front_face = true; // arbitary
        rec.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> &AABB {
        self.boundry.bounding_box()
    }
}
