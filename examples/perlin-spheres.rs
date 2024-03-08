use path_tracer::{
    camera::CameraBuilder,
    hittable::{HittableList, Sphere},
    material::Material,
    math::{prelude::*, Point3},
    texture::PerlinTexture,
};
use std::sync::Arc;

fn main() {
    let mut world = HittableList::new();

    let pertex = Arc::new(PerlinTexture::new(1.));

    world.push(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Material::lambertian(pertex.clone()),
    ));
    world.push(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Material::lambertian(pertex),
    ));

    let camera = CameraBuilder::default()
        .image_width(800)
        .image_height(450)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .lookfrom(Point3::new(13., 2., 3.))
        .lookat(Point3::origin())
        .build();

    camera.render(&world, "output/perlin-spheres.png").unwrap();
}
