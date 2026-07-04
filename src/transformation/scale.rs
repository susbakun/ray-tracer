use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    prelude::HittableType,
    ray::Ray,
};

// scale uniformly
pub struct Scale {
    object: HittableType,
    factor: f64,
    bbox: AABB,
}

impl Scale {
    pub fn new(object: HittableType, factor: f64) -> Self {
        let bbox = *object.bounding_box() * factor;

        assert!(factor > 0.0);

        Self {
            object,
            factor,
            bbox,
        }
    }
}

impl Hittable for Scale {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.

        let origin = ray.origin() / self.factor;

        let direction = ray.dir() / self.factor;

        let scaled_ray = Ray::new_with_time(origin, direction, ray.time());

        if !self.object.hit(&scaled_ray, ray_t, rec) {
            return false;
        }

        rec.p = rec.p * self.factor;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
