use rand::rngs::ThreadRng;

use crate::{
    interval::Interval,
    prelude::{Dot, random_number_range},
    vector::{Point3, Vec3},
};

pub struct Perlin {
    point_count: usize,
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let mut rng = rand::rng();

        let mut rand_vec = Vec::new();
        let rand_vec_range = Interval::new(-1.0, 1.0);
        for _ in 0..point_count {
            rand_vec.push(Vec3::new(
                random_number_range(&mut rng, rand_vec_range),
                random_number_range(&mut rng, rand_vec_range),
                random_number_range(&mut rng, rand_vec_range),
            ));
        }

        let mut perm_x = Vec::with_capacity(point_count);
        let mut perm_y = Vec::with_capacity(point_count);
        let mut perm_z = Vec::with_capacity(point_count);

        Self::perlin_generate_perm(&mut perm_x, point_count, &mut rng);
        Self::perlin_generate_perm(&mut perm_y, point_count, &mut rng);
        Self::perlin_generate_perm(&mut perm_z, point_count, &mut rng);

        Self {
            point_count,
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn turb(&self, point: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = point.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor();
        let j = point.y().floor();
        let k = point.z().floor();

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x
                        [((i as isize + di as isize) & 255) as usize]
                        ^ self.perm_y[((j as isize + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k as isize + dk as isize) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm(p: &mut Vec<usize>, point_count: usize, rng: &mut ThreadRng) {
        for i in 0..point_count {
            p.push(i);
        }

        Self::permute(p, point_count, rng);
    }

    fn permute(p: &mut Vec<usize>, n: usize, rng: &mut ThreadRng) {
        for i in (0..n - 1).rev() {
            let range = Interval::new(0.0, i as f64);
            let target = random_number_range(rng, range) as usize;
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - (2.0 * u));
        let vv = v * v * (3.0 - (2.0 * v));
        let ww = w * w * (3.0 - (2.0 * w));

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let weight_v = Vec3::new(u - i, v - j, w - k);

                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize].dot(weight_v)
                }
            }
        }

        accum
    }
}
