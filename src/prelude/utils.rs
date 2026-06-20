use rand::{RngExt, rngs::ThreadRng};

use crate::interval::Interval;

pub fn random_number01(rng: &mut ThreadRng) -> f64 {
    rng.random::<f64>()
}

pub fn random_number_range(rng: &mut ThreadRng, range: Interval) {
    rng.random_range(range.min..range.max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rand_01() {
        let mut rng = rand::rng();
        let f = random_number01(&mut rng);

        println!("{f}");
    }
}
