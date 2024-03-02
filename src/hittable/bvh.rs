use super::{Hittable, HittableList};
use crate::ray::Ray;
use cgmath::{prelude::*, Point3, Vector3};
use rand::prelude::*;
use std::{ops::Range, sync::Arc};

#[derive(Clone, Copy)]
pub(crate) struct Aabb {
    pub(crate) center: Point3<f64>,
    pub(crate) half_extents: Vector3<f64>,
}

impl Aabb {
    pub(crate) fn empty() -> Self {
        Self {
            center: Point3::origin(),
            half_extents: Vector3::from([-f64::INFINITY; 3]),
        }
    }

    pub(crate) fn from_min_max(minimum: Point3<f64>, maximum: Point3<f64>) -> Self {
        let center = minimum.midpoint(maximum);
        let half_extents = 0.5 * (maximum - minimum);

        Self {
            center,
            half_extents,
        }
    }

    pub(crate) fn covering(a: &Self, b: &Self) -> Self {
        let minimum = Point3::new(
            a.min().x.min(b.min().x),
            a.min().y.min(b.min().y),
            a.min().z.min(b.min().z),
        );

        let maximum = Point3::new(
            a.max().x.max(b.max().x),
            a.max().y.max(b.max().y),
            a.max().z.max(b.max().z),
        );

        Self::from_min_max(minimum, maximum)
    }

    pub(crate) fn padding(&self) -> Self {
        const DELTA: f64 = 5e-5;

        let mut half_extents = self.half_extents;

        if half_extents[0] < DELTA {
            half_extents[0] += DELTA;
        }
        if half_extents[1] < DELTA {
            half_extents[1] += DELTA;
        }
        if half_extents[2] < DELTA {
            half_extents[2] += DELTA;
        }

        Self {
            center: self.center,
            half_extents,
        }
    }

    #[inline]
    pub(crate) fn min(&self) -> Point3<f64> {
        self.center - self.half_extents
    }

    #[inline]
    pub(crate) fn max(&self) -> Point3<f64> {
        self.center + self.half_extents
    }

    pub(crate) fn hit(&self, ray: &Ray, mut range: Range<f64>) -> bool {
        let mut t0: [f64; 3] = (self.min() - ray.origin)
            .div_element_wise(ray.direction)
            .into();
        let mut t1: [f64; 3] = (self.max() - ray.origin)
            .div_element_wise(ray.direction)
            .into();

        for i in 0..3 {
            if ray.direction[i] < 0. {
                std::mem::swap(&mut t0[i], &mut t1[i]);
            }

            range.start = range.start.max(t0[i]);
            range.end = range.end.min(t1[i]);
        }

        !range.is_empty()
    }
}

struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    aabb: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<super::HitPayload> {
        if !self.aabb.hit(ray, range.clone()) {
            return None;
        }

        let left = self.left.hit(ray, range.clone());
        let right = if let Some(l) = &left {
            self.right.hit(ray, range.start..l.t)
        } else {
            self.right.hit(ray, range)
        };

        if right.is_some() {
            right
        } else {
            left
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}

impl BvhNode {
    fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>]) -> Arc<Self> {
        let axis = rand::thread_rng().gen_range(0..3);

        let aabb_cmp = |a: Aabb, b: Aabb| match axis {
            0 => a.min().x.partial_cmp(&b.min().x).unwrap(),
            1 => a.min().y.partial_cmp(&b.min().y).unwrap(),
            2 => a.min().z.partial_cmp(&b.min().z).unwrap(),
            _ => unreachable!(),
        };

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if aabb_cmp(objects[0].bounding_box(), objects[1].bounding_box()).is_le() {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_by(|o1, o2| aabb_cmp(o1.bounding_box(), o2.bounding_box()));

                let mid = objects.len() / 2;
                let left = Self::new(&mut objects[0..mid]);
                let right = Self::new(&mut objects[mid..]);

                (left as _, right as _)
            }
        };

        let aabb = Aabb::covering(&left.bounding_box(), &right.bounding_box());

        Arc::new(Self { left, right, aabb })
    }
}

pub struct Bvh {
    root: Arc<BvhNode>,
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, range: Range<f64>) -> Option<super::HitPayload> {
        self.root.hit(ray, range)
    }

    fn bounding_box(&self) -> Aabb {
        self.root.aabb
    }
}

impl Bvh {
    pub fn from_list(world: &mut HittableList) -> Self {
        Self {
            root: BvhNode::new(&mut world.objects),
        }
    }
}
