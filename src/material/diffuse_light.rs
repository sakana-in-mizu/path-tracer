use crate::{hittable::HitPayload, texture::Texture, Color};
use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseLightMaterial {
    emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLightMaterial {
    #[allow(private_bounds)]
    pub fn new<T: Texture + Send + Sync + 'static>(emit: Arc<T>) -> Self {
        Self { emit }
    }

    pub(crate) fn emitted(&self, payload: &HitPayload) -> Color {
        self.emit.value(payload.u, payload.v, &payload.point)
    }
}
