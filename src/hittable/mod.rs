use crate::{material::Material, ray::Ray};
use cgmath::{prelude::*, Point3, Vector3};
use std::ops::Range;
use std::sync::Arc;

mod bvh;
mod quad;
mod shpere;
mod transform;

pub use bvh::Bvh;
pub use quad::Quad;
pub use shpere::Sphere;
pub use transform::Transform;

use bvh::Aabb;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitPayload>;

    fn bounding_box(&self) -> Aabb;
}

pub(crate) struct HitPayload<'a> {
    pub(crate) point: Point3<f64>,
    pub(crate) normal: Vector3<f64>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,

    pub(crate) u: f64,
    pub(crate) v: f64,

    pub(crate) material: &'a Material,
}

impl<'a> HitPayload<'a> {
    fn new(
        ray: &Ray,
        point: Point3<f64>,
        outward_normal: Vector3<f64>,
        t: f64,
        u: f64,
        v: f64,
        material: &'a Material,
    ) -> Self {
        let front_face = outward_normal.dot(ray.direction) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,

            u,
            v,

            material,
        }
    }
}

pub struct HittableList {
    pub(crate) objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub(crate) aabb: Aabb,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitPayload> {
        let hit_payload: Option<HitPayload> = self.objects.iter().fold(None, |closest, object| {
            let end = if let Some(closest) = &closest {
                closest.t
            } else {
                range.end
            };
            if let Some(payload) = object.hit(ray, range.start..end) {
                Some(payload)
            } else {
                closest
            }
        });

        hit_payload
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            aabb: Aabb::empty(),
        }
    }

    #[allow(private_bounds)]
    pub fn push<H: Hittable + Send + Sync + 'static>(&mut self, object: Arc<H>) {
        self.aabb = Aabb::covering(&self.aabb, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.aabb = Aabb::empty();
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}
