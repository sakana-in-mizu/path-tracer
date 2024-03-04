use cgmath::Rotation3;
use pbr::{
    camera::CameraBuilder,
    hittable::{Bvh, ConstantMedium, HittableList, Quad, Sphere, Transform},
    material::Material,
    math::{Deg, Point3, Quaternion, Vector3},
    texture::{ImageTexture, PerlinTexture},
    Color,
};
use rand::prelude::*;
use std::sync::Arc;

const IMAGE_WIDTH: u32 = 800;
const SAMPLES_PER_PIXEL: u32 = 10000;
const MAX_DEPTH: u32 = 40;

fn main() {
    let mut rng = rand::thread_rng();

    let mut boxes1 = HittableList::new();
    let ground = Material::lambertian(Color::new(0.48, 0.83, 0.53).into());

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;

            let x0 = -1000.0 + w * i as f64;
            let z0 = -1000.0 + w * j as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.push(Quad::cuboid(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let boxes1 = Arc::new(Bvh::from_list(&mut boxes1));

    let mut world = HittableList::new();
    world.push(boxes1);

    let light = Material::diffuse_light(Color::new(7., 7., 7.).into());
    world.push(Quad::new(
        Point3::new(123., 554., 147.),
        Vector3::new(300., 0., 0.),
        Vector3::new(0., 0., 265.),
        light,
    ));

    let sphere_material = Material::lambertian(Color::new(0.7, 0.3, 0.1).into());
    world.push(Sphere::new(
        Point3::new(400., 400., 200.),
        50.,
        sphere_material,
    ));

    world.push(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Material::dielectric(1.5),
    ));
    world.push(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Material::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Material::dielectric(1.5),
    );
    world.push(boundary.clone());
    world.push(ConstantMedium::new(
        boundary.clone(),
        Color::new(0.2, 0.4, 0.9).into(),
        0.2,
    ));
    let boundary = Sphere::new(Point3::from([0.; 3]), 5000., Material::dielectric(1.5));
    world.push(ConstantMedium::new(
        boundary,
        Color::new(1., 1., 1.).into(),
        0.0001,
    ));

    let emat = Material::lambertian(ImageTexture::new("images/earthmap.jpg").unwrap().into());
    world.push(Sphere::new(Point3::new(400., 200., 400.), 100., emat));
    let pertext = PerlinTexture::new(0.1);
    world.push(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Material::lambertian(pertext.into()),
    ));

    let mut boxes2 = HittableList::new();
    let white = Material::lambertian(Color::from([0.73; 3]).into());
    let ns = 1000;
    for _ in 0..ns {
        let center = Point3::new(
            rng.gen_range(0.0..165.0),
            rng.gen_range(0.0..165.0),
            rng.gen_range(0.0..165.0),
        );
        boxes2.push(Sphere::new(center, 10., white.clone()));
    }
    let boxes2 = Transform::new(
        Arc::new(Bvh::from_list(&mut boxes2)),
        Vector3::new(-100., 270., 395.),
        Quaternion::from_angle_y(Deg(15.)),
    );
    world.push(boxes2);

    let world = Bvh::from_list(&mut world);

    let camera = CameraBuilder::default()
        .image_width(IMAGE_WIDTH)
        .image_height(IMAGE_WIDTH)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_depth(MAX_DEPTH)
        .background(Color::from([0.; 3]))
        .vfov(40.)
        .lookfrom(Point3::new(478., 278., -600.))
        .lookat(Point3::new(278., 278., 0.))
        .vup(Vector3::unit_y())
        .build();

    camera
        .render(&world, "output/integrated_scene.png")
        .unwrap();
}
