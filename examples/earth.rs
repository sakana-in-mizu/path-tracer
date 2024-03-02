use pbr::{
    camera::CameraBuilder,
    hittable::Sphere,
    material::Material,
    math::{prelude::*, Point3},
    texture::ImageTexture,
};

fn main() {
    let earth_texture = ImageTexture::new("images/earthmap.jpg").unwrap();
    let earth_surface = Material::lambertian(earth_texture.into());
    let globe = Sphere::new(Point3::origin(), 2., earth_surface);

    let camera = CameraBuilder::default()
        .image_width(800)
        .image_height(450)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .lookfrom(Point3::new(0., 0., 12.))
        .lookat(Point3::origin())
        .build();

    camera.render(globe.as_ref(), "output/earth.png").unwrap();
}
