use pbr::{
    camera::CameraBuilder,
    hittable::{Bvh, HittableList, Quad},
    material::Material,
    math::{prelude::*, Point3, Vector3},
    Color,
};

fn main() {
    let mut world = HittableList::new();

    let left_red = Material::lambertian(Color::new(1., 0.2, 0.2).into());
    let back_green = Material::lambertian(Color::new(0.2, 1., 0.2).into());
    let right_blue = Material::lambertian(Color::new(0.2, 0.2, 1.).into());
    let upper_orange = Material::lambertian(Color::new(1., 0.5, 0.).into());
    let lower_teal = Material::lambertian(Color::new(0.2, 0.8, 0.8).into());

    world.push(Quad::new(
        Point3::new(-3., -2., 5.),
        Vector3::new(0., 0., -4.),
        Vector3::new(0., 4., 0.),
        left_red,
    ));
    world.push(Quad::new(
        Point3::new(-2., -2., 0.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 4., 0.),
        back_green,
    ));
    world.push(Quad::new(
        Point3::new(3., -2., 1.),
        Vector3::new(0., 0., 4.),
        Vector3::new(0., 4., 0.),
        right_blue,
    ));
    world.push(Quad::new(
        Point3::new(-2., 3., 1.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., 4.),
        upper_orange,
    ));
    world.push(Quad::new(
        Point3::new(-2., -3., 5.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., -4.),
        lower_teal,
    ));

    let world = Bvh::from_list(&mut world);

    let camera = CameraBuilder::default()
        .image_width(600)
        .image_height(600)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(80.)
        .lookfrom(Point3::new(0., 0., 9.))
        .lookat(Point3::origin())
        .build();

    camera.render(&world, "output/quads.png").unwrap();
}
