use pbr::{
    camera::CameraBuilder,
    hittable::{Bvh, ConstantMedium, HittableList, Quad, Transform},
    material::Material,
    math::{prelude::*, Deg, Point3, Quaternion, Vector3},
    Color,
};

fn main() {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05).into());
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73).into());
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15).into());
    let light = Material::diffuse_light(Color::new(7., 7., 7.).into());

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
        Point3::new(113., 554., 127.),
        Vector3::new(330., 0., 0.),
        Vector3::new(0., 0., 305.),
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
        white.clone(),
    ));

    world.push(ConstantMedium::new(
        Transform::new(
            Quad::cuboid(
                Point3::new(0., 0., 0.),
                Point3::new(165., 330., 165.),
                white.clone(),
            ),
            Vector3::new(265., 0., 295.),
            Quaternion::from_angle_y(Deg(15.)),
        ),
        Color::from([0.; 3]).into(),
        0.01,
    ));
    world.push(ConstantMedium::new(
        Transform::new(
            Quad::cuboid(
                Point3::new(0., 0., 0.),
                Point3::new(165., 165., 165.),
                white,
            ),
            Vector3::new(130., 0., 65.),
            Quaternion::from_angle_y(Deg(-18.)),
        ),
        Color::from([1.; 3]).into(),
        0.01,
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

    camera.render(&world, "output/cornell-smoke.png").unwrap();
}
