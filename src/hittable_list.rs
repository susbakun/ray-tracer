use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let interval = Interval::from(ray_t.min, closest_so_far);

            if obj.hit(ray, interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                std::mem::swap(rec, &mut temp_rec);
            }
        }

        hit_anything
    }
}
