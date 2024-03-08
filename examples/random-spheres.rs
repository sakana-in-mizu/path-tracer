use path_tracer::{
    camera::CameraBuilder,
    hittable::{Bvh, HittableList, Sphere},
    material::Material,
    math::{prelude::*, Point3},
    Color,
};
use rand::{prelude::*, rngs::StdRng};
use std::time::Instant;

fn main() {
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(2077);

    let mut world = HittableList::new();

    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5).into());
    world.push(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                i as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                j as f64 + 0.9 * rng.gen::<f64>(),
            );

            if center.distance(Point3::new(4., 0.2, 0.)) <= 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                let albedo = rng.gen::<Color>().mul_element_wise(rng.gen::<Color>());
                world.push(Sphere::new(
                    center,
                    0.2,
                    Material::lambertian(Color::from(albedo).into()),
                ));
            } else if choose_mat < 0.95 {
                let albedo = 0.5 * rng.gen::<Color>() + Color::from([0.5; 3]);
                let fuzz = rng.gen_range(0.0..0.5);
                world.push(Sphere::new(center, 0.2, Material::metal(albedo, fuzz)));
            } else {
                world.push(Sphere::new(center, 0.2, Material::dielectric(1.5)));
            };
        }
    }

    let material1 = Material::dielectric(1.5);
    world.push(Sphere::new(Point3::new(0., 1., 0.), 1., material1));

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1).into());
    world.push(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Point3::new(4., 1., 0.), 1., material3));

    let camera = CameraBuilder::default()
        .image_width(800)
        .image_height(450)
        .vfov(20.)
        .lookfrom(Point3::new(13., 2., 3.))
        .lookat(Point3::new(3.36, 0.52, 0.78))
        .defocus_angle(0.6)
        .samples_per_pixel(500)
        .max_depth(50)
        .build();

    let world = Bvh::from_list(&mut world);

    let start_time = Instant::now();
    camera.render(&world, "output/random-spheres.png").unwrap();
    let end_time = Instant::now();
    let elapsed_millis = (end_time - start_time).as_millis();

    println!("{} ms", elapsed_millis);
}
