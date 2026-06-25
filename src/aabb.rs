use crate::{
    interval::{self, Interval},
    ray::Ray,
    vector::Point3,
};

#[derive(Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_point(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::from(a.x(), b.x())
        } else {
            Interval::from(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::from(a.y(), b.y())
        } else {
            Interval::from(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::from(a.z(), b.z())
        } else {
            Interval::from(b.z(), a.z())
        };
        Self { x, y, z }
    }

    pub fn from_boxes(box1: &Self, box2: &Self) -> Self {
        let x = Interval::sort_two_intervals(box1.x, box2.x);
        let y = Interval::sort_two_intervals(box1.y, box2.y);
        let z = Interval::sort_two_intervals(box1.z, box2.z);

        Self::new(x, y, z)
    }

    pub const fn axis_interval(&self, n: u8) -> Interval {
        if n == 1 {
            return self.y;
        }
        if n == 2 {
            return self.z;
        }
        self.x
    }

    pub fn longest_axis(&self) -> u8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            }
            return 2;
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            }
            return 2;
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut local_t = *ray_t;

        let ray_origin = ray.origin();
        let ray_dir = ray.dir();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ray_dir_ax = ray_dir.get_axis(axis);

            let adinv = 1.0 / ray_dir_ax;

            let ray_orgin_ax = ray_origin.get_axis(axis);

            let t0 = (ax.min - ray_orgin_ax) * adinv;
            let t1 = (ax.max - ray_orgin_ax) * adinv;

            if t0 < t1 {
                if t0 > local_t.min {
                    local_t.min = t0;
                }
                if t1 < local_t.max {
                    local_t.max = t1;
                }
            } else {
                if t1 > local_t.min {
                    local_t.min = t1;
                }
                if t0 < local_t.max {
                    local_t.max = t0;
                }
            }

            if local_t.max <= local_t.min {
                return false;
            }
        }
        true
    }
}

pub const EMPTY: AABB = AABB::new(interval::EMPTY, interval::EMPTY, interval::EMPTY);
pub const UNIVERSE: AABB = AABB::new(interval::UNIVERSE, interval::UNIVERSE, interval::UNIVERSE);
