use rand::{RngExt, rngs::ThreadRng};

use crate::{interval::Interval, prelude::Dot, vector::Vec3};

pub fn random_number01(rng: &mut ThreadRng) -> f64 {
    rng.random::<f64>()
}

pub fn random_number_range(rng: &mut ThreadRng, range: Interval) -> f64 {
    rng.random_range(range.min..range.max)
}

pub fn random_vector01(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(
        random_number01(rng),
        random_number01(rng),
        random_number01(rng),
    )
}

pub fn random_vector_range(rng: &mut ThreadRng, range: Interval) -> Vec3 {
    Vec3::new(
        random_number_range(rng, range),
        random_number_range(rng, range),
        random_number_range(rng, range),
    )
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let range = Interval::from(-1.0, 1.0);
        let p = random_vector_range(rng, range);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq;
        }
    }
}

pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
    let on_unit_hemisphere = random_unit_vector(rng);
    if on_unit_hemisphere.dot(*normal) > 0.0 {
        on_unit_hemisphere
    } else {
        -on_unit_hemisphere
    }
}
