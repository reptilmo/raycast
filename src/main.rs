use std::io::Write;
use std::time::Instant;

extern crate rand;

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod sphere;
mod utils;
mod vec3;
mod world;

use camera::*;
use material::*;
use renderer::*;
use sphere::*;
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

    let (width, height) =
        utils::parse_pair::<usize>(&args[2], 'x').expect("Failed to parse image size");

    let aspect_ratio = width as f64 / height as f64;
    let samples_per_pixel: u32 = 500;
    let focal_length = 2.0;

    let camera = Camera::new(
        Point3::new(0.0, 1.0, 4.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        focal_length,
        aspect_ratio,
    );

    let mut world = World::new();
    random_scene(&mut world);

    let the_renderer = Renderer::new(width, height, samples_per_pixel);

    let now = Instant::now();
    let pixels = the_renderer.draw_scene(&camera, &world);
    println!("Renderer.draw_scene: {}ms", now.elapsed().as_millis());

    utils::write_image(&args[1], renderer::pixels_to_bytes(&pixels), width, height);
}

fn random_scene(world: &mut World) {
    world.add_object(Sphere::new(
        Point3::new(0.0, -10000.0, 0.0),
        10000.0,
        Material::Diffuse(Color::new(0.5, 0.5, 0.5)),
    ));

    world.add_object(Sphere::new(
        Point3::new(-1.0, 0.5, -1.0),
        0.5,
        Material::Diffuse(Color::new(0.1, 0.8, 0.2)),
    ));

    world.add_object(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        0.5,
        Material::Dielectric(1.1),
    ));

    world.add_object(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        -0.45,
        Material::Dielectric(1.1),
    ));

    world.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.5,
        Material::Metalic(Color::new(0.8, 0.6, 0.2), 0.0),
    ));

    world.add_object(Sphere::new(
        Point3::new(0.1, 0.15, -0.2),
        0.15,
        Material::Dielectric(1.5),
    ));

    world.add_object(Sphere::new(
        Point3::new(0.1, 0.15, -0.2),
        -0.15,
        Material::Dielectric(1.5),
    ));

    world.add_object(Sphere::new(
        Point3::new(-0.5, 0.1, -0.2),
        0.1,
        Material::Dielectric(1.5),
    ));

    world.add_object(Sphere::new(
        Point3::new(-0.7, 0.05, -0.6),
        0.05,
        Material::Metalic(Color::new(0.9, 0.1, 0.1), 0.2),
    ));
}
