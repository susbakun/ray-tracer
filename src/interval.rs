use std::{
    f64::INFINITY,
    ops::{Add, Mul},
};

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn sort_two_intervals(a: Interval, b: Interval) -> Self {
        let min = if a.min < b.min { a.min } else { b.min };
        let max = if a.max > b.max { a.max } else { b.max };

        Interval { min, max }
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.min + rhs, self.max + rhs)
    }
}

impl Mul<f64> for Interval {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.min * rhs, self.max * rhs)
    }
}

pub const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
