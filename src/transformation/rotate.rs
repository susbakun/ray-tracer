use std::f64::INFINITY;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    prelude::HittableType,
    ray::Ray,
    vector::{Point3, Vec3},
};

pub struct RotateX {
    object: HittableType,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl RotateX {
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

                    let new_z = cos_theta * z - sin_theta * y;
                    let new_y = cos_theta * y + sin_theta * z;

                    let tester = Vec3::new(x, new_y, new_z);

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

impl Hittable for RotateX {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            ray.origin().x(),
            (self.cos_theta * ray.origin().y()) + (self.sin_theta * ray.origin().z()),
            (self.cos_theta * ray.origin().z()) - (self.sin_theta * ray.origin().y()),
        );

        let direction = Vec3::new(
            ray.dir().x(),
            (self.cos_theta * ray.dir().y()) + (self.sin_theta * ray.dir().z()),
            (self.cos_theta * ray.dir().z()) - (self.sin_theta * ray.dir().y()),
        );

        let rotated_ray = Ray::new_with_time(origin, direction, ray.time());

        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            rec.p.x(),
            (self.cos_theta * rec.p.y()) - (self.sin_theta * rec.p.z()),
            (self.cos_theta * rec.p.z()) + (self.sin_theta * rec.p.y()),
        );

        rec.normal = Point3::new(
            rec.normal.x(),
            (self.cos_theta * rec.normal.y()) - (self.sin_theta * rec.normal.z()),
            (self.cos_theta * rec.normal.z()) + (self.sin_theta * rec.normal.y()),
        );

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

pub struct RotateZ {
    object: HittableType,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl RotateZ {
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

                    let new_x = cos_theta * x - sin_theta * y;
                    let new_y = cos_theta * y + sin_theta * x;

                    let tester = Vec3::new(new_x, new_y, z);

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

impl Hittable for RotateZ {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().y()),
            (self.cos_theta * ray.origin().y()) + (self.sin_theta * ray.origin().x()),
            ray.origin().z(),
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.dir().x()) - (self.sin_theta * ray.dir().y()),
            (self.cos_theta * ray.dir().y()) + (self.sin_theta * ray.dir().x()),
            ray.dir().z(),
        );

        let rotated_ray = Ray::new_with_time(origin, direction, ray.time());

        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.y()),
            (self.cos_theta * rec.p.y()) - (self.sin_theta * rec.p.x()),
            rec.p.z(),
        );

        rec.normal = Point3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.y()),
            (self.cos_theta * rec.normal.y()) - (self.sin_theta * rec.normal.x()),
            rec.normal.z(),
        );

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
