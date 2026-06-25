use crate::vector::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            tm: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, dir: Vec3, time: f64) -> Self {
        Self {
            origin,
            dir,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        (self.dir * t) + self.origin
    }
}
