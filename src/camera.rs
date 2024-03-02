use crate::{color_to_rgb, hittable::Hittable, random_in_unit_disk, ray::Ray, Color};
use cgmath::{prelude::*, Point3, Vector3};
use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    background: Color,

    center: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    pixel00_loc: Point3<f64>,

    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

impl Camera {
    #[allow(private_bounds)]
    pub fn render<H: Hittable + Sync>(&self, world: &H, path: &str) -> image::ImageResult<()> {
        let total = self.image_width * self.image_height;
        let buf: Vec<_> = (0..total)
            .into_par_iter()
            .progress_count(total as u64)
            .map(|idx| {
                let i = idx % self.image_width;
                let j = idx / self.image_width;
                let color = (0..self.samples_per_pixel)
                    .map(|_| {
                        self.get_ray(i, j)
                            .color(world, &self.background, self.max_depth)
                    })
                    .sum::<Color>()
                    / (self.samples_per_pixel as f64);

                color_to_rgb(color)
            })
            .flat_map(|rgb| rgb)
            .collect();

        image::save_buffer(
            path,
            &buf,
            self.image_width,
            self.image_height,
            image::ColorType::Rgb8,
        )?;

        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = self.center + self.defocus_disk_sample();
        let direction = pixel_sample - origin;

        Ray { origin, direction }
    }

    fn pixel_sample_square(&self) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = random_in_unit_disk();
        p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}

pub struct CameraBuilder {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background: Color,

    pub vfov: f64,
    pub lookat: Point3<f64>,
    pub lookfrom: Point3<f64>,
    pub vup: Vector3<f64>,

    pub defocus_angle: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 600,
            image_height: 600,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.,
            lookat: Point3::origin(),
            lookfrom: Point3::new(0., 0., -1.),
            vup: Vector3::unit_y(),

            defocus_angle: 0.,

            background: Color::new(0.7, 0.8, 1.),
        }
    }
}

impl CameraBuilder {
    #[inline]
    pub fn image_width(&mut self, image_width: u32) -> &mut Self {
        self.image_width = image_width;
        self
    }

    #[inline]
    pub fn image_height(&mut self, image_height: u32) -> &mut Self {
        self.image_height = image_height;
        self
    }

    #[inline]
    pub fn samples_per_pixel(&mut self, samples_per_pixel: u32) -> &mut Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    #[inline]
    pub fn max_depth(&mut self, max_depth: u32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    #[inline]
    pub fn vfov(&mut self, vfov: f64) -> &mut Self {
        self.vfov = vfov;
        self
    }

    #[inline]
    pub fn lookat(&mut self, lookat: Point3<f64>) -> &mut Self {
        self.lookat = lookat;
        self
    }

    #[inline]
    pub fn lookfrom(&mut self, lookfrom: Point3<f64>) -> &mut Self {
        self.lookfrom = lookfrom;
        self
    }

    #[inline]
    pub fn vup(&mut self, vup: Vector3<f64>) -> &mut Self {
        self.vup = vup;
        self
    }

    #[inline]
    pub fn defocus_angle(&mut self, defocus_angle: f64) -> &mut Self {
        self.defocus_angle = defocus_angle;
        self
    }

    #[inline]
    pub fn background(&mut self, background: Color) -> &mut Self {
        self.background = background;
        self
    }

    pub fn build(&self) -> Camera {
        let aspect_ratio = (self.image_width as f64) / (self.image_height as f64);
        let center = self.lookfrom;

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let focus_dist = self.lookat.distance(self.lookfrom);
        let viewport_height = 2. * focus_dist * (self.vfov.to_radians() / 2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / (self.image_width as f64);
        let pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = center - (focus_dist * w) - 0.5 * (viewport_u + viewport_v);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (self.defocus_angle.to_radians() / 2.).tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera {
            image_width: self.image_width,
            image_height: self.image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            background: self.background,

            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,

            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
