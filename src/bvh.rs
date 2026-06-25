use std::{cmp::Ordering, rc::Rc};

use rand::rngs::ThreadRng;

use crate::{
    aabb::{self, AABB},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    prelude::random_number_range,
    ray::Ray,
};

pub struct BVH {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVH {
    pub fn bvh_node(list: &mut HittableList, rng: &mut ThreadRng) -> Self {
        let start = 0;
        let end = list.objects.len();
        Self::build_tree(&mut list.objects, start, end, rng)
    }

    fn build_tree(
        objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        rng: &mut ThreadRng,
    ) -> Self {
        let mut bbox = aabb::EMPTY;
        for i in start..end {
            bbox = AABB::from_boxes(&bbox, objects[i].bounding_box())
        }
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };

        let span = end - start;

        let left;
        let right;

        if span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
        } else if span == 2 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start + 1]);
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + span / 2;

            left = Rc::new(Self::build_tree(objects, start, mid, rng));
            right = Rc::new(Self::build_tree(objects, mid, end, rng));
        }

        Self { left, right, bbox }
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: u8) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, rec);

        let rihgt_t = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self
            .right
            .hit(ray, &mut Interval::from(ray_t.min, rihgt_t), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
