use crate::{
    aabb::{self, AABB},
    hittable::Hittable,
    matrix::Matrix,
    prelude::{Cross, MaterialType, UV},
    vector::{Point3, Vec3},
};

pub struct Triangle {
    // vertex positions
    p0: Point3,
    p1: Point3,
    p2: Point3,
    // normals per vertex
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
    // uvs per vertex
    uv0: UV,
    uv1: UV,
    uv2: UV,
    mat: MaterialType,
    bbox: AABB,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: MaterialType) -> Self {
        let p0_p1 = p1 - p0;
        let p0_p2 = p2 - p0;

        let n0 = p0_p1.cross(p0_p2).unit_vector();
        let n1 = p0_p1.cross(p0_p2).unit_vector();
        let n2 = p0_p1.cross(p0_p2).unit_vector();

        let uv0 = UV::new(0.0, 0.0);
        let uv1 = UV::new(1.0, 0.0);
        let uv2 = UV::new(0.0, 1.0);

        let mut triangle = Self {
            p0,
            p1,
            p2,
            n0,
            n1,
            n2,
            uv0,
            uv1,
            uv2,
            mat,
            bbox: aabb::EMPTY,
        };

        triangle.set_bounding_box();

        triangle
    }

    // used when the uv and normals are provided
    pub fn new_with_normals_uvs(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        mat: MaterialType,
        n0: Vec3,
        n1: Vec3,
        n2: Vec3,
        uv0: UV,
        uv1: UV,
        uv2: UV,
    ) -> Self {
        let mut triangle = Self {
            p0,
            p1,
            p2,
            n0,
            n1,
            n2,
            uv0,
            uv1,
            uv2,
            mat,
            bbox: aabb::EMPTY,
        };
        triangle.set_bounding_box();
        triangle
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::from_point(self.p0, self.p1);
        let bbox_diagonal2 = AABB::from_point(self.p0, self.p2);
        let bbox = AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2);

        self.bbox = bbox;
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

        let w = 1.0 - u - v;

        let mut normal = (self.n0 * w) + (self.n1 * u) + (self.n2 * v);
        normal = normal.unit_vector();

        let intersection = ray.at(t);
        rec.p = intersection;
        rec.t = t;
        rec.material = self.mat.clone();
        rec.normal = normal;
        rec.set_face_normal(ray, normal);
        rec.u = (self.uv0.u * w) + (self.uv1.u * u) + (self.uv2.u * v);
        rec.v = (self.uv0.v * w) + (self.uv1.v * u) + (self.uv2.v * v);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
