use path_tracer::{
    camera::CameraBuilder,
    hittable::{HittableList, Quad, Sphere},
    material::Material,
    math::{Point3, Vector3},
    texture::PerlinTexture,
    Color,
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

    let difflight = Material::diffuse_light(Color::new(4., 4., 4.).into());
    world.push(Quad::new(
        Point3::new(3., 1., -2.),
        Vector3::new(2., 0., 0.),
        Vector3::new(0., 2., 0.),
        difflight.clone(),
    ));
    world.push(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        difflight,
    ));

    let camera = CameraBuilder::default()
        .image_width(800)
        .image_height(450)
        .samples_per_pixel(100)
        .max_depth(50)
        .background(Color::from([0.; 3]))
        .vfov(20.)
        .lookfrom(Point3::new(26., 3., 6.))
        .lookat(Point3::new(0., 2., 0.))
        .build();

    camera.render(&world, "output/light.png").unwrap();
}
