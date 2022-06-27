use std::boxed::*;
use std::io::Write;

extern crate rand;
use rand::prelude::*;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod world;

use camera::*;
use material::*;
use ray::*;
use sphere::*;
use utils::*;
use vec3::*;
use world::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        writeln!(
            std::io::stderr(),
            "Usage: {} <png file> <image size> ...",
            args[0]
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} myfile.png 800x600 ...",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bounds = utils::parse_pair::<usize>(&args[2], 'x').expect("Failed to parse image size");
    let mut pixels = vec![Pixel { r: 0, g: 0, b: 0 }; bounds.0 * bounds.1];

    render(&mut pixels, bounds.0, bounds.1);

    write_image(&args[1], &pixels, bounds);
}

fn random_scene(world: &mut World) {
    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Diffuse(Color::new(0.5, 0.5, 0.5)),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-1.0, 0.5, -1.0),
        0.5,
        Material::Dielectric(1.2),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-1.0, 0.5, -1.0),
        -0.45,
        Material::Dielectric(1.2),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        0.5,
        Material::Diffuse(Color::new(1.0, 0.2, 0.1)),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.5,
        Material::Metalic(Color::new(0.8, 0.6, 0.2), 0.0),
    )));
    /*
    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();

            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    world.add_object(Box::<Sphere>::new(Sphere::new(
                        Point3::new(-0.15, -0.2, -0.5),
                        0.1,
                        Material::Diffuse(Color::new(0.4, 0.2, 0.1)),
                    )));
                } else if choose_material < 0.95 {
                    world.add_object(Box::<Sphere>::new(Sphere::new(
                        Point3::new(-0.15, -0.2, -0.5),
                        0.1,
                        Material::Metalic(Color::new(0.4, 0.2, 0.1), 0.2),
                    )));
                }
            }
        }
    }
    */

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.1, 0.15, -0.2),
        0.15,
        Material::Dielectric(1.5),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.1, 0.15, -0.2),
        -0.15,
        Material::Dielectric(1.5),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-0.5, 0.1, -0.2),
        0.1,
        Material::Dielectric(1.3),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-0.7, 0.05, -0.6),
        0.05,
        Material::Metalic(Color::new(0.0, 0.9, 0.1), 0.2),
    )));
}

fn render(pixels: &mut [Pixel], width: usize, height: usize) {
    let fov = 90.0;
    let aspect_ration = width as f64 / height as f64;
    let samples_per_pixel: usize = 40;
    let fov_factor = f64::tan(fov * 0.5 * PI / 180.0);

    let camera = Camera::new(
        Point3::new(3.0, 1.0, 2.0),
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut world = World::new();
    random_scene(&mut world);

    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut color = Color::new(1.0, 1.0, 1.0);
            for _ in 0..samples_per_pixel {
                let u = (2.0 * ((x as f64 + rng.gen::<f64>()) / width as f64) - 1.0)
                    * fov_factor
                    * aspect_ration;
                let v = 1.0 - 2.0 * ((y as f64 + rng.gen::<f64>()) / height as f64) * fov_factor;

                let r = camera.cast_ray(u, v);
                color += color_ray(&world, &r, 30);
            }

            let c = color / samples_per_pixel as f64; //FIXME:

            pixels[y * width + x] = Pixel {
                r: (255.0 * utils::clamp(0.0, 1.0, c.x)) as u8,
                g: (255.0 * utils::clamp(0.0, 1.0, c.y)) as u8,
                b: (255.0 * utils::clamp(0.0, 1.0, c.z)) as u8,
            };
        }
    }
}

fn color_ray(world: &World, ray: &Ray, bounce: i32) -> Color {
    if bounce <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.test_camera_ray(ray) {
        None => (),
        Some(hit) => match hit.material.scatter(ray, &hit) {
            None => return Color::new(0.0, 0.0, 0.0),
            Some((attenuation, scattered_ray)) => {
                return color_ray(world, &scattered_ray, bounce - 1) * attenuation
            }
        },
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    let color_1 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
    let color_2 = Vec3::new(0.5, 0.7, 1.0) * t;
    color_1 + color_2
}
