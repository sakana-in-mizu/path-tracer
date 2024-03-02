pub mod camera;
pub mod hittable;
pub mod material;
pub mod texture;

mod ray;

use math::{prelude::*, Point3, Vector2, Vector3};
use rand::Rng;
use std::ops::{Add, Mul};
use texture::Texture;

pub use cgmath as math;
pub type Color = Vector3<f64>;

impl Texture for Color {
    fn value(&self, _u: f64, _v: f64, _p: &Point3<f64>) -> Color {
        *self
    }
}

fn color_to_rgb(color: Color) -> [u8; 3] {
    const GAMMA: f64 = 2.2;

    let r = (color.x.powf(GAMMA.recip()) * 255.) as u8;
    let g = (color.y.powf(GAMMA.recip()) * 255.) as u8;
    let b = (color.z.powf(GAMMA.recip()) * 255.) as u8;

    [r, g, b]
}

fn rgb_to_color(rgb: [u8; 3]) -> Color {
    const GAMMA: f64 = 2.2;

    let scale = 255.0_f64.recip();
    let x = (scale * (rgb[0] as f64)).powf(GAMMA);
    let y = (scale * (rgb[1] as f64)).powf(GAMMA);
    let z = (scale * (rgb[2] as f64)).powf(GAMMA);

    Color::new(x, y, z)
}

fn random_in_unit_disk() -> Vector2<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let vec = 2. * rng.gen::<Vector2<f64>>() - Vector2::from([1.; 2]);

        if vec.magnitude2() < 1. {
            return vec;
        }
    }
}

fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let vec = 2. * rng.gen::<Vector3<f64>>() - Vector3::from([1.; 3]);

        if vec.magnitude2() < 1. {
            return vec;
        }
    }
}

fn random_unit_vector() -> Vector3<f64> {
    return random_in_unit_sphere().normalize();
}

fn near_zero(vec: &Vector3<f64>) -> bool {
    const EPS: f64 = 1e-8;
    vec.x.abs() < EPS && vec.y.abs() < EPS && vec.z.abs() < EPS
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(n) * n
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv.dot(n)).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude2()).abs()).sqrt() * n;
    return r_out_perp + r_out_parallel;
}

fn lerp<V: Add<Output = V> + Mul<f64, Output = V>>(v0: V, v1: V, t: f64) -> V {
    v0 * (1. - t) + v1 * t
}
