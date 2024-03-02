use super::{bvh::Aabb, HitPayload, Hittable};
use crate::{material::Material, ray::Ray};
use cgmath::{prelude::*, Point3, Vector3};
use std::sync::Arc;

pub struct Quad {
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    normal: Vector3<f64>,
    aabb: Aabb,

    material: Material,
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitPayload> {
        let s = ray.origin - self.q;
        let s1 = ray.direction.cross(self.v);
        let s2 = s.cross(self.u);

        let det = s1.dot(self.u);
        if det.abs() < 1e-8 {
            return None;
        }

        let rdet = det.recip();

        let t = rdet * s2.dot(self.v);
        if !range.contains(&t) {
            return None;
        }

        let beta = rdet * s1.dot(s);
        let gamma = rdet * s2.dot(ray.direction);

        if beta < 0. || beta > 1. || gamma < 0. || gamma > 1. {
            return None;
        }

        let point = ray.at(t);

        Some(HitPayload::new(
            ray,
            point,
            self.normal,
            t,
            beta,
            gamma,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}

impl Quad {
    pub fn new(q: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, material: Material) -> Arc<Self> {
        let normal = u.cross(v).normalize();

        let p = q + u + v;
        let minimum = Point3::new(q.x.min(p.x), q.y.min(p.y), q.z.min(p.z));
        let maximum = Point3::new(q.x.max(p.x), q.y.max(p.y), q.z.max(p.z));
        let aabb = Aabb::from_min_max(minimum, maximum).padding();

        Arc::new(Self {
            q,
            u,
            v,
            normal,
            aabb,

            material,
        })
    }
}
