use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    prelude::{Cross, Dot},
    ray::Ray,
    vector::{Point3, Vec3},
};

pub struct Quad {
    // quad origin
    q: Point3,
    // basis vectors
    u: Vec3,
    v: Vec3,
    // plane normal
    n: Vec3,
    // n.p for each point on the plane
    d: f64,
    w: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self::set_bounding_box(q, u, v, mat)
    }

    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let bbox_diagonal1 = AABB::from_point(q, q + u + v);
        let bbox_diagonal2 = AABB::from_point(q + u, q + v);

        let bbox = AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2);

        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);

        let w = n / n.dot(n);

        Self {
            q,
            u,
            v,
            n: normal,
            d,
            w,
            mat,
            bbox,
        }
    }

    fn is_interior(alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_inteval = Interval::new(0.0, 1.0);

        if !unit_inteval.contains(alpha) || !unit_inteval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;
        true
    }
}

pub fn create_box(a: Point3, b: Point3, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    // front
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));
    // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    // back
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));
    // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    // bottom
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    )));

    sides
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = self.n.dot(ray.dir());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.n.dot(ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.material = self.mat.clone();
        rec.set_face_normal(ray, self.n);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
