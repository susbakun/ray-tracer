use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::prelude::{Cross, Dot};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        // return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - (*n * self.dot(*n)) * 2.0
    }
}

pub type Point3 = Vec3;

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Dot for Vec3 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, rhs: Self) -> Self::Output {
        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.x * rhs.z) - (self.z * rhs.x);
        let z = (self.x * rhs.y) - (self.y * rhs.x);

        Self::new(x, -y, z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
