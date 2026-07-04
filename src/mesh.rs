use crate::{
    aabb::{self, AABB},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    shape::Triangle,
};

pub struct Mesh {
    triangles: Vec<Triangle>,
    bbox: AABB,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: vec![],
            bbox: aabb::EMPTY,
        }
    }

    pub fn from(triangle: Triangle) -> Self {
        let mut mesh = Mesh::new();
        mesh.add(triangle);
        mesh
    }

    pub fn add(&mut self, triangle: Triangle) {
        // update aabb of objects
        self.bbox = AABB::from_boxes(&self.bbox, triangle.bounding_box());
        // and then push the obj to the list
        self.triangles.push(triangle);
    }

    pub fn clear(&mut self) {
        self.triangles.clear()
    }
}

impl Hittable for Mesh {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &mut crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.triangles {
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
