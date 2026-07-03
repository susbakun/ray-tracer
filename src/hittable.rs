use std::{f64::INFINITY, sync::Arc};

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Lambertian,
    prelude::*,
    ray::Ray,
    vector::{Point3, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: MaterialType,
    pub u: f64,
    pub v: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            material: Arc::new(Lambertian::default()),
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

pub struct Translate {
    object: HittableType,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: HittableType, offset: Vec3) -> Self {
        let bbox = *object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(ray.origin() - self.offset, ray.dir(), ray.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateY {
    object: HittableType,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: HittableType, theta: f64) -> Self {
        let rad_theta = theta.to_radians();

        let cos_theta = rad_theta.cos();
        let sin_theta = rad_theta.sin();

        let mut bbox = object.bounding_box().clone();

        let mut min = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        let mut max = Point3::new(INFINITY, INFINITY, INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bbox.axis_interval(0).max + (1.0 - i) * bbox.axis_interval(0).min;
                    let y = j * bbox.axis_interval(1).max + (1.0 - j) * bbox.axis_interval(1).min;
                    let z = k * bbox.axis_interval(2).max + (1.0 - k) * bbox.axis_interval(2).min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min.set_axis(c, min.get_axis(c).min(tester.get_axis(c)));
                        max.set_axis(c, max.get_axis(c).max(tester.get_axis(c)));
                    }
                }
            }
        }

        bbox = AABB::from_point(min, max);

        Self {
            object,
            cos_theta,
            sin_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()),
            ray.origin().y(),
            (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.dir().x()) - (self.sin_theta * ray.dir().z()),
            ray.dir().y(),
            (self.sin_theta * ray.dir().x()) + (self.cos_theta * ray.dir().z()),
        );

        let rotated_ray = Ray::new_with_time(origin, direction, ray.time());

        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Point3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
