use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    prelude::HittableType,
    ray::Ray,
    vector::Vec3,
};

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
