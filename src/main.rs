use std::boxed::*;
use std::fs::File;
use std::io::Write;

extern crate image;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;

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

fn ray_color(world: &World, ray: &Ray, bounce: i32) -> Color {
    if bounce <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.cast_camera_ray(ray) {
        None => (),
        Some(hit) => match hit.material.scatter(ray, &hit) {
            None => return Color::new(0.0, 0.0, 0.0),
            Some((attenuation, scattered_ray)) => {
                return ray_color(world, &scattered_ray, bounce - 1) * attenuation
            }
        },
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    let color_1 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
    let color_2 = Vec3::new(0.5, 0.7, 1.0) * t;
    color_1 + color_2
}

fn write_image(filename: &str, pixels: &[Pixel], bounds: (usize, usize)) {
    let file = File::create(filename).unwrap();
    let encoder = PngEncoder::new(file);
    let bytes = utils::pixels_to_bytes(pixels);

    encoder.write_image(&bytes, bounds.0 as u32, bounds.1 as u32, ColorType::Rgba8);
}

fn render(pixels: &mut [Pixel], width: usize, height: usize) {
    let fov = 90.0;
    let aspect_ration = width as f64 / height as f64;
    let origin = Point3::new(0.0, 0.0, 10.0);
    let fov_factor = f64::tan(fov / 2.0 * PI / 180.0);
    let samples_per_pixel: usize = 100;

    let mut world = World::new();
    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Diffuse(Color::new(0.7, 0.7, 0.7)),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.5),
        0.5,
        Material::Metalic(Color::new(1.0, 0.0, 0.0), 0.5),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.5),
        -0.45,
        Material::Dielectric(1.07),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.5),
        0.5,
        Material::Diffuse(Color::new(0.0, 1.0, 0.0)),
    )));
    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.5),
        0.5,
        Material::Metalic(Color::new(0.8, 0.6, 0.2), 0.0),
    )));
    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.1, -0.2, -0.5),
        0.15,
        Material::Dielectric(1.5),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(0.1, -0.2, -0.5),
        -0.15,
        Material::Dielectric(1.5),
    )));

    world.add_object(Box::<Sphere>::new(Sphere::new(
        Point3::new(-0.6, -0.3, -0.5),
        0.1,
        Material::Dielectric(1.3),
    )));

    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut color = Color::new(1.0, 1.0, 1.0);
            for _ in 0..samples_per_pixel {
                let u = (2.0 * ((x as f64 + rng.gen::<f64>()) / width as f64) - 1.0)
                    * fov_factor
                    * aspect_ration;
                let v = 1.0 - 2.0 * ((y as f64 + rng.gen::<f64>()) / height as f64) * fov_factor;

                let r = Ray::new(origin, Vec3::new(u, v, -1.0) - origin);
                color += ray_color(&world, &r, 30);
            }

            let c = color / samples_per_pixel as f64; //FIXME:

            pixels[y * width + x] = Pixel {
                r: (255.0 * utils::clamp(0.0, 1.0, c.x)) as u8,
                g: (255.0 * utils::clamp(0.0, 1.0, c.y)) as u8,
                b: (255.0 * utils::clamp(0.0, 1.0, c.z)) as u8,
                a: 255,
            };
        }
    }
}

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
    let mut pixels = vec![
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };
        bounds.0 * bounds.1
    ];

    render(&mut pixels, bounds.0, bounds.1);

    write_image(&args[1], &pixels, bounds);
}
