use std::rc::Rc;

use crate::{
    aabb::{self, AABB},
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: aabb::EMPTY,
        }
    }

    pub fn from(obj: Rc<dyn Hittable>) -> Self {
        let mut hl = Self::new();
        hl.add(obj);
        hl
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        // update aabb of objects
        self.bbox = AABB::from_boxes(&self.bbox, obj.bounding_box());
        // and then push the obj to the list
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let mut interval = Interval::new(ray_t.min, closest_so_far);

            if obj.hit(ray, &mut interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                std::mem::swap(rec, &mut temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
