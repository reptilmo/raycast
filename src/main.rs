use std::fs::File;
use std::io::Write;
use std::boxed::*;

extern crate image;
use image::ImageEncoder;
use image::ColorType;
use image::codecs::png::PngEncoder;

extern crate rand;
use rand::prelude::*;


mod utils;
mod vec3;
mod ray;
mod hittable;
mod world;
mod sphere;

use utils::*;
use vec3::*;
use ray::*;
use world::*;
use sphere::*;

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let random = Vec3::random_in_unit_sphere();
    if random.dot(*normal) > 0.0 {
        return random
    }

    -random
}

fn ray_color(world: &World, ray: &Ray, bounce: &mut i32) -> Color {

    if *bounce <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }

    match world.cast_camera_ray(ray) {
        None => (),
        Some(hit) => {
            let bounce_point = hit.point + random_in_hemisphere(&hit.normal);
            let bounced_ray = Ray::new(hit.point, bounce_point - hit.point);
            *bounce = *bounce - 1;
            return ray_color(world, &bounced_ray, bounce) * 0.5;
        },
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    let color_1 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
    let color_2 = Vec3::new(0.5, 0.7, 1.0) * t;
    return color_1 + color_2
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
    let origin = Point3::new(0.0, 0.0, 0.0);
    let fov_factor = f64::tan(fov / 2.0 * PI / 180.0);
    let samples_per_pixe: usize = 100;

    let mut world = World::new();
    world.add_object(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Color::new(0.7, 0.7, 0.7))));
    world.add_object(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, 0.0, -5.0), 0.5, Color::new(1.0, 0.0, 0.0))));
    world.add_object(Box::<Sphere>::new(Sphere::new(Point3::new(2.0, 1.0, -8.0), 2.0, Color::new(0.0, 1.0, 0.0))));
    world.add_object(Box::<Sphere>::new(Sphere::new(Point3::new(-1.5, -0.5, -3.0), 0.7, Color::new(0.0, 0.0, 1.0))));
    //world.add_object(Box::<Sphere>::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Color::new(1.0, 0.0, 0.0))));

    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut color = Color::new(1.0, 1.0, 1.0);
            for _ in 0..samples_per_pixe {
                // let u = (2.0 * ((x as f64  + 0.5) / width as f64) - 1.0) * fov_factor * aspect_ration;
                // let v = 1.0 - 2.0 * ((y as f64 + 0.5) / height as f64) * fov_factor;
                let u = (2.0 * ((x as f64  + rng.gen::<f64>()) / width as f64) - 1.0) * fov_factor * aspect_ration;
                let v = 1.0 - 2.0 * ((y as f64 + rng.gen::<f64>()) / height as f64) * fov_factor;
                
                let r = Ray::new(origin, Vec3::new(u, v, -1.0) - origin);

                let mut bounce = 30;
                color += ray_color(&world, &r, &mut bounce);
            }

            let c = color / samples_per_pixe as f64; //FIXME:
            
            pixels[y * width + x] = Pixel{
                r: (255.0 * utils::clamp(0.0, 1.0, c.x)) as u8, 
                g: (255.0 * utils::clamp(0.0, 1.0, c.y)) as u8, 
                b: (255.0 * utils::clamp(0.0, 1.0, c.z)) as u8, 
                a: 255
            };
        }
    }
}

fn main() {
    let args :Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        writeln!(std::io::stderr(), "Usage: {} <png file> <image size> ...", args[0]).unwrap();
        writeln!(std::io::stderr(), "Example: {} myfile.png 800x600 ...", args[0]).unwrap();
        std::process::exit(1); 
    }

    let bounds = utils::parse_pair::<usize>(&args[2], 'x').expect("Failed to parse image size");
    let mut pixels = vec![Pixel{r: 0, g: 0, b: 0, a: 255}; bounds.0 * bounds.1];

    render(&mut pixels, bounds.0, bounds.1);

    write_image(&args[1], &pixels, bounds);
}
