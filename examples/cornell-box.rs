use pbr::{
    camera::CameraBuilder,
    hittable::{Bvh, HittableList, Quad},
    material::Material,
    math::{prelude::*, Point3, Vector3},
    Color,
};

fn main() {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05).into());
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73).into());
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15).into());
    let light = Material::diffuse_light(Color::new(15., 15., 15.).into());

    world.push(Quad::new(
        Point3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        green,
    ));
    world.push(Quad::new(
        Point3::origin(),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        red,
    ));
    world.push(Quad::new(
        Point3::new(343., 554., 332.),
        Vector3::new(-130., 0., 0.),
        Vector3::new(0., 0., -105.),
        light,
    ));
    world.push(Quad::new(
        Point3::origin(),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
        white.clone(),
    ));
    world.push(Quad::new(
        Point3::from([555.; 3]),
        Vector3::new(-555., 0., 0.),
        Vector3::new(0., 0., -555.),
        white.clone(),
    ));
    world.push(Quad::new(
        Point3::new(0., 0., 555.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        white,
    ));

    let camera = CameraBuilder::default()
        .image_width(600)
        .image_height(600)
        .samples_per_pixel(200)
        .max_depth(50)
        .background(Color::from([0.; 3]))
        .vfov(40.)
        .lookfrom(Point3::new(278., 278., -800.))
        .lookat(Point3::new(278., 278., 0.))
        .build();

    let world = Bvh::from_list(&mut world);

    camera.render(&world, "output/cornell-box.png").unwrap();
}
