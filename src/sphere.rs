use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::prelude::Dot;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new_stationary(static_center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        let rvec = Point3::new(radius, radius, radius);
        let bbox = AABB::from_point(static_center - rvec, static_center + rvec);

        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            material,
            bbox,
        }
    }
    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let radius = radius.max(0.0);

        let rvec = Point3::new(radius, radius, radius);
        let box1 = AABB::from_point(center1 - rvec, center1 + rvec);
        let box2 = AABB::from_point(center2 - rvec, center2 + rvec);
        let bbox = AABB::from_boxes(&box1, &box2);

        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc = current_center - ray.origin();
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
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Rc::clone(&self.material);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
