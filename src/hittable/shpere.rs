use super::{bvh::Aabb, HitPayload, Hittable, Range};
use crate::{material::Material, ray::Ray};
use cgmath::{prelude::*, Point3, Vector3};
use std::sync::Arc;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    aabb: Aabb,

    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Arc<Self> {
        let rvec = Vector3::from([radius; 3]);
        let aabb = Aabb::from_min_max(center - rvec, center + rvec);

        Arc::new(Sphere {
            center,
            radius,
            aabb,

            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<HitPayload> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        let theta = (-normal.y).acos();
        let phi = (-normal.z).atan2(normal.x) + std::f64::consts::PI;

        let u = phi * 0.5 * std::f64::consts::FRAC_1_PI;
        let v = theta * std::f64::consts::FRAC_1_PI;

        Some(HitPayload::new(
            &ray,
            point,
            normal,
            root,
            u,
            v,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
