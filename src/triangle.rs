use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::Hittable,
    material::Material,
    matrix::Matrix,
    prelude::Cross,
    vector::{Point3, Vec3},
};

pub struct Triangle {
    p0: Point3,
    p1: Point3,
    p2: Point3,
    normal: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: AABB,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self::set_bounding_box(p0, p1, p2, mat)
    }

    fn set_bounding_box(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let bbox_diagonal1 = AABB::from_point(p0, p1);
        let bbox_diagonal2 = AABB::from_point(p0, p2);
        let bbox = AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2);

        let p0_p1 = p1 - p0;
        let p0_p2 = p2 - p0;

        let n = p0_p1.cross(p0_p2);
        let normal = n.unit_vector();

        Self {
            p0,
            p1,
            p2,
            normal,
            mat,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &mut crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let right_side = ray.origin() - self.p0;

        let denominator =
            Matrix::new(vec![-ray.dir(), self.p1 - self.p0, self.p2 - self.p0]).determinant();

        if denominator.abs() < 1e-8 {
            return false;
        }

        let t = Matrix::new(vec![right_side, self.p1 - self.p0, self.p2 - self.p0]).determinant()
            / denominator;
        let u = Matrix::new(vec![-ray.dir(), right_side, self.p2 - self.p0]).determinant()
            / denominator;
        let v = Matrix::new(vec![-ray.dir(), self.p1 - self.p0, right_side]).determinant()
            / denominator;

        if !ray_t.contains(t) {
            return false;
        }

        if u < 0.0 || v < 0.0 || u + v > 1.0 {
            return false;
        }

        let intersection = ray.at(t);
        rec.p = intersection;
        rec.t = t;
        rec.u = u;
        rec.v = v;
        rec.material = self.mat.clone();
        rec.normal = self.normal;
        rec.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
