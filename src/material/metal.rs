use crate::{hittable::HitPayload, random_unit_vector, ray::Ray, reflect, Color};
use cgmath::prelude::*;

#[derive(Clone)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        MetalMaterial { albedo, fuzz }
    }

    pub(crate) fn scatter(&self, r_in: &Ray, payload: &HitPayload) -> Option<(Color, Ray)> {
        let reflected = reflect(r_in.direction.normalize(), payload.normal);
        let scattered_direction = reflected + self.fuzz * random_unit_vector();

        if scattered_direction.dot(payload.normal) <= 0. {
            return None;
        }

        let scattered = Ray {
            origin: payload.point,
            direction: scattered_direction,
        };

        Some((self.albedo, scattered))
    }
}
