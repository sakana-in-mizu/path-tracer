use super::{bvh::Aabb, HitPayload, Hittable, Range};
use crate::{material::Material, ray::Ray, texture::Texture};
use cgmath::{prelude::*, Vector3};
use rand::prelude::*;
use std::sync::Arc;

pub struct ConstantMedium {
    neg_inv_density: f64,
    boundary: Arc<dyn Hittable + Send + Sync>,
    phase_function: Material,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitPayload> {
        let mut rng = rand::thread_rng();

        let mut payload1 = match self.boundary.hit(ray, f64::NEG_INFINITY..f64::INFINITY) {
            Some(payload) => payload,
            _ => return None,
        };

        let mut payload2 = match self.boundary.hit(ray, payload1.t + 0.0001..f64::INFINITY) {
            Some(payload) => payload,
            _ => return None,
        };

        if payload1.t < range.start {
            payload1.t = range.start
        }
        if payload2.t > range.end {
            payload2.t = range.end
        }

        if payload1.t < 0. {
            payload1.t = 0.;
        }

        if payload1.t >= payload2.t {
            return None;
        }

        let ray_length = ray.direction.magnitude();
        let distance_inside_boundary = (payload2.t - payload1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = payload1.t + hit_distance / ray_length;
        let point = ray.at(t);

        Some(HitPayload {
            point,
            normal: Vector3::unit_z(),
            t,
            front_face: false,
            u: 0.0,
            v: 0.0,
            material: &self.phase_function,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}

impl ConstantMedium {
    #[allow(private_bounds)]
    pub fn new<H: Hittable + Send + Sync + 'static, T: Texture + Send + Sync + 'static>(
        boundary: Arc<H>,
        phase: Arc<T>,
        density: f64,
    ) -> Arc<Self> {
        Arc::new(Self {
            neg_inv_density: -density.recip(),
            boundary: boundary,
            phase_function: Material::isotropic(phase),
        })
    }
}
