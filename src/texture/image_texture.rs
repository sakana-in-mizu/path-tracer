use super::Texture;
use crate::{lerp, rgb_to_color, Color};
use cgmath::Point3;
use image::{DynamicImage, GenericImageView, Pixel};

pub struct ImageTexture {
    image: DynamicImage,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3<f64>) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let x = u * self.image.width() as f64;
        let y = v * self.image.height() as f64;

        // texel center: (i + 0.5, j + 0.5)
        let a = (x - 0.5).floor() + 0.5;
        let b = (y - 0.5).floor() + 0.5;

        let i1 = a as u32;
        let j1 = b as u32;

        let i2 = (i1 + 1).min(self.image.width() - 1);
        let j2 = (j1 + 1).min(self.image.height() - 1);

        let c00 = rgb_to_color(self.image.get_pixel(i1, j1).to_rgb().0);
        let c10 = rgb_to_color(self.image.get_pixel(i2, j1).to_rgb().0);
        let c01 = rgb_to_color(self.image.get_pixel(i1, j2).to_rgb().0);
        let c11 = rgb_to_color(self.image.get_pixel(i2, j2).to_rgb().0);

        let t = x - a as f64;
        let s = y - b as f64;

        let c0 = lerp(c00, c10, t);
        let c1 = lerp(c01, c11, t);

        lerp(c0, c1, s)
    }
}

impl ImageTexture {
    pub fn new(path: &str) -> std::io::Result<Self> {
        use image::io::Reader as ImageReader;

        let image = ImageReader::open(path)?.decode().unwrap();

        Ok(Self { image })
    }
}
