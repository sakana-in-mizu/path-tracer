use super::{bvh::Aabb, HitPayload, Hittable, Range};
use crate::ray::Ray;
use cgmath::{prelude::*, Point3, Quaternion, Vector3};
use std::sync::Arc;

pub struct Transform {
    translation: Vector3<f64>,
    rotation: Quaternion<f64>,
    object: Arc<dyn Hittable + Send + Sync>,
}

impl Hittable for Transform {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitPayload> {
        let inverse_rot = self.rotation.invert();
        let origin = inverse_rot.rotate_point(ray.origin - self.translation);
        let direction = inverse_rot.rotate_vector(ray.direction);

        let equivalent_ray = Ray { origin, direction };

        if let Some(mut payload) = self.object.hit(&equivalent_ray, range) {
            payload.point = self.rotation.rotate_point(payload.point) + self.translation;
            payload.normal = self.rotation.rotate_vector(payload.normal);

            Some(payload)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        let aabb = self.object.bounding_box();
        let factor = [-1., 1.];

        let mut minimum = Point3::from([std::f64::INFINITY; 3]);
        let mut maximum = Point3::from([std::f64::NEG_INFINITY; 3]);
        for i in factor {
            for j in factor {
                for k in factor {
                    let point = aabb.center
                        + Vector3::new(
                            i * aabb.half_extents.x,
                            j * aabb.half_extents.y,
                            k * aabb.half_extents.z,
                        );
                    let point = self.rotation.rotate_point(point) + self.translation;

                    minimum.x = minimum.x.min(point.x);
                    minimum.y = minimum.y.min(point.y);
                    minimum.z = minimum.z.min(point.z);

                    maximum.x = maximum.x.max(point.x);
                    maximum.y = maximum.y.max(point.y);
                    maximum.z = maximum.z.max(point.z);
                }
            }
        }

        Aabb::from_min_max(minimum, maximum)
    }
}

impl Transform {
    #[allow(private_bounds)]
    pub fn new<H: Hittable + Send + Sync + 'static>(
        object: Arc<H>,
        translation: Vector3<f64>,
        rotation: Quaternion<f64>,
    ) -> Arc<Self> {
        Arc::new(Self {
            translation,
            rotation,
            object,
        })
    }
}
