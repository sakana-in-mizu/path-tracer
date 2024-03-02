use crate::Color;

mod image_texture;
mod perlin;

use cgmath::Point3;

pub use image_texture::ImageTexture;
pub use perlin::PerlinTexture;

pub(crate) trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3<f64>) -> Color;
}
