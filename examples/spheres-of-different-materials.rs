use path_tracer::{
    camera::CameraBuilder,
    hittable::{HittableList, Sphere},
    material::Material,
    math::Point3,
    Color,
};

fn main() {
    let mut world = HittableList::new();
    let material_ground = Material::lambertian(Color::new(0.8, 0.8, 0.0).into());
    let material_center = Material::lambertian(Color::new(0.1, 0.2, 0.5).into());
    let material_left = Material::dielectric(1.5);
    let material_right = Material::metal(Color::new(0.8, 0.8, 0.8), 0.2);

    world.push(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.push(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    world.push(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));
    world.push(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    world.push(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        material_left.clone(),
    ));

    let camera = CameraBuilder::default()
        .image_width(800)
        .image_height(450)
        .lookat(Point3::new(0., 0., -0.95))
        .lookfrom(Point3::new(0., 0., 2.5))
        .vfov(20.)
        .defocus_angle(10.)
        .samples_per_pixel(100)
        .max_depth(50)
        .build();

    camera
        .render(&world, "output/spheres-of-different-materials.png")
        .unwrap();
}
