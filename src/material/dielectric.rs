use crate::{hittable::HitPayload, ray::Ray, reflect, reflectance, refract, Color};
use cgmath::prelude::*;
use rand::prelude::*;

#[derive(Clone)]
pub struct DielectricMaterial {
    ir: f64,
}

impl DielectricMaterial {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    pub(crate) fn scatter(&self, r_in: &Ray, payload: &HitPayload) -> Option<(Color, Ray)> {
        let attenuation = Color::from([1.; 3]);
        let refraction_ratio = if payload.front_face {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction.dot(payload.normal)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let mut rng = rand::thread_rng();
        let scatter_direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
                reflect(unit_direction, payload.normal)
            } else {
                refract(unit_direction, payload.normal, refraction_ratio)
            };

        let scattered = Ray {
            origin: payload.point,
            direction: scatter_direction,
        };

        return Some((attenuation, scattered));
    }
}
