use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::prelude::Dot;
use crate::vector::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.dir().length_squared();
        let h = ray.dir().dot(oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (h * h) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let mut root = (h - discriminant.sqrt()) / a;
        if !ray_t.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Rc::clone(&self.material);

        true
    }
}
