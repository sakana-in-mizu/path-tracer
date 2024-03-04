use crate::{hittable::HitPayload, random_unit_vector, ray::Ray, texture::Texture, Color};
use std::sync::Arc;

#[derive(Clone)]
pub struct IsotropicMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl IsotropicMaterial {
    #[allow(private_bounds)]
    pub fn new<T: Texture + Send + Sync + 'static>(albedo: Arc<T>) -> Self {
        Self { albedo }
    }

    pub(crate) fn scatter(&self, payload: &HitPayload) -> Option<(Color, Ray)> {
        let scattered = Ray {
            origin: payload.point,
            direction: random_unit_vector(),
        };
        let attenuation = self.albedo.value(payload.u, payload.v, &payload.point);

        Some((attenuation, scattered))
    }
}
