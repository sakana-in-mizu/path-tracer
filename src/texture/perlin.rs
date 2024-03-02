use super::Texture;
use crate::{lerp, Color};
use cgmath::{prelude::*, Point3, Vector3};
use rand::prelude::*;

struct Perlin {
    ranvec: Box<[Vector3<f64>; Self::POINT_COUNT]>,
    perm_x: Box<[usize; Self::POINT_COUNT]>,
    perm_y: Box<[usize; Self::POINT_COUNT]>,
    perm_z: Box<[usize; Self::POINT_COUNT]>,
}

impl Perlin {
    const POINT_COUNT: usize = 1 << 8;

    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut ranvec = Box::new([Vector3::zero(); Self::POINT_COUNT]);
        ranvec.iter_mut().for_each(|x| {
            *x = (2. * rng.gen::<Vector3<f64>>() - Vector3::from([1.; 3])).normalize()
        });

        let mut perm_x = Box::new([0; Self::POINT_COUNT]);
        perm_x.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        perm_x.shuffle(&mut rng);

        let mut perm_y = Box::new([0; Self::POINT_COUNT]);
        perm_y.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        perm_y.shuffle(&mut rng);

        let mut perm_z = Box::new([0; Self::POINT_COUNT]);
        perm_z.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        perm_z.shuffle(&mut rng);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn noise(&self, p: &Point3<f64>) -> f64 {
        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();

        let u = p.x - i;
        let v = p.y - j;
        let w = p.z - k;

        let i = i as i64 as usize;
        let j = j as i64 as usize;
        let k = k as i64 as usize;

        let sample = move |ii, jj, kk| {
            let vec = self.ranvec[self.perm_x[ii % Self::POINT_COUNT]
                ^ self.perm_y[jj % Self::POINT_COUNT]
                ^ self.perm_z[kk % Self::POINT_COUNT]];

            let weight_v = Vector3::new(
                u - (ii - i) as f64,
                v - (jj - j) as f64,
                w - (kk - k) as f64,
            );

            weight_v.dot(vec)
        };

        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let f00 = lerp(sample(i, j, k), sample(i + 1, j, k), uu);
        let f10 = lerp(sample(i, j + 1, k), sample(i + 1, j + 1, k), uu);
        let f01 = lerp(sample(i, j, k + 1), sample(i + 1, j, k + 1), uu);
        let f11 = lerp(sample(i, j + 1, k + 1), sample(i + 1, j + 1, k + 1), uu);

        let f0 = lerp(f00, f10, vv);
        let f1 = lerp(f01, f11, vv);

        lerp(f0, f1, ww)
    }

    fn turb(&self, p: &Point3<f64>, depth: u32) -> f64 {
        let (acc, _, _) = (0..depth).fold((0., *p, 1.0), |(acc, temp_p, weight), _| {
            let acc = acc + weight * self.noise(&temp_p);
            let temp_p = 2. * temp_p;
            let weight = 0.5 * weight;

            (acc, temp_p, weight)
        });

        acc.abs()
    }
}

pub struct PerlinTexture {
    perlin: Perlin,
    scale: f64,
}

impl Texture for PerlinTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3<f64>) -> Color {
        let s = self.scale * p;
        0.5 * (1. + (s.z + 10. * self.perlin.turb(&s, 7)).sin()) * Color::from([1.; 3])
    }
}

impl PerlinTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}
