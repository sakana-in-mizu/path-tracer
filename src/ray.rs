use crate::{hittable::Hittable, Color};
use cgmath::{prelude::*, Point3, Vector3};

pub(crate) struct Ray {
    pub(crate) origin: Point3<f64>,
    pub(crate) direction: Vector3<f64>,
}

impl Ray {
    pub(crate) fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub(crate) fn color<H: Hittable>(&self, world: &H, background: &Color, depth: u32) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(payload) = world.hit(self, 0.001..f64::INFINITY) {
            let color_from_emission = payload.material.emitted(&payload);

            if let Some((attenuation, scattered)) = payload.material.scatter(self, &payload) {
                let color_from_scatter =
                    attenuation.mul_element_wise(scattered.color(world, background, depth - 1));

                return color_from_scatter + color_from_emission;
            }

            color_from_emission
        } else {
            return *background;
        }
    }
}
