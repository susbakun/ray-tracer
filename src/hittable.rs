use std::rc::Rc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::{Lambertian, Material},
    prelude::Dot,
    ray::Ray,
    vector::{Point3, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            material: Rc::new(Lambertian::default()),
            t: 0.0,
            p: Point3::default(),
            normal: Vec3::default(),
            front_face: false,
            u: 0.0,
            v: 0.0,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;
}
