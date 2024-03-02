use crate::{
    hittable::HitPayload, near_zero, random_unit_vector, ray::Ray, texture::Texture, Color,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct LambertianMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl LambertianMaterial {
    #[allow(private_bounds)]
    pub fn new<T: Texture + Send + Sync + 'static>(albedo: Arc<T>) -> Self {
        Self { albedo }
    }

    pub(crate) fn scatter(&self, payload: &HitPayload) -> Option<(Color, Ray)> {
        let mut scatter_direction = payload.normal + random_unit_vector();
        if near_zero(&scatter_direction) {
            scatter_direction = payload.normal;
        }

        let scattered = Ray {
            origin: payload.point,
            direction: scatter_direction,
        };

        Some((
            self.albedo.value(payload.u, payload.v, &payload.point),
            scattered,
        ))
    }
}
