use super::camera::Camera;
use super::ray::Ray;
use super::vec3::*;
use super::world::World;

use rand::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn pixels_to_bytes(pixels: &[Pixel]) -> &[u8] {
    let p: *const u8 = pixels.as_ptr() as *const u8;
    let n: usize = 3 * pixels.len();
    unsafe { std::slice::from_raw_parts(p, n) }
}

fn clamp(lo: f64, hi: f64, value: f64) -> f64 {
    f64::max(lo, f64::min(hi, value))
}

pub struct Renderer {
    width: usize,
    height: usize,
    samples_per_pixel: u32,
    sampling_factor: f64,
}

impl Renderer {
    pub fn new(width: usize, height: usize, samples_per_pixel: u32) -> Self {
        let sampling_factor = 1.0 / samples_per_pixel as f64;

        Self {
            width,
            height,
            samples_per_pixel,
            sampling_factor,
        }
    }

    pub fn draw_scene(&self, camera: &Camera, world: &World) -> Vec<Pixel> {
        let mut pixels = vec![Pixel { r: 0, g: 0, b: 0 }; self.width * self.height];
        let scanlines: Vec<(usize, &mut [Pixel])> =
            pixels.chunks_mut(self.width).enumerate().collect();

        scanlines
            .into_par_iter()
            .for_each(|(y, scanline)| self.draw_scanline(camera, world, y, scanline));

        pixels
    }

    fn draw_scanline(&self, camera: &Camera, world: &World, y: usize, scanline: &mut [Pixel]) {
        let mut rng = thread_rng();

        for x in 0..self.width {
            let mut color = Color::new(1.0, 1.0, 1.0);

            for _ in 0..self.samples_per_pixel {
                let u = ((x as f64 + rng.gen::<f64>()) / self.width as f64) - 0.5;
                let v = 0.5 - ((y as f64 + rng.gen::<f64>()) / self.height as f64);

                let ray = camera.cast_ray(u, v);
                color += self.color_ray(&world, &ray, 64);
            }

            let c = color * self.sampling_factor;

            scanline[x] = Pixel {
                r: (255.0 * clamp(0.0, 1.0, c.x)) as u8,
                g: (255.0 * clamp(0.0, 1.0, c.y)) as u8,
                b: (255.0 * clamp(0.0, 1.0, c.z)) as u8,
            };
        }
    }

    fn color_ray(&self, world: &World, ray: &Ray, bounce: i32) -> Color {
        if bounce <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.test_camera_ray(ray) {
            None => (),
            Some(hit) => match hit.material.scatter(ray, &hit) {
                None => return Color::new(0.0, 0.0, 0.0),
                Some((attenuation, scattered_ray)) => {
                    return self.color_ray(world, &scattered_ray, bounce - 1) * attenuation
                }
            },
        }

        let t = 0.5 * (ray.direction.y + 1.0);
        let color_1 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
        let color_2 = Vec3::new(0.5, 0.7, 1.0) * t;
        color_1 + color_2
    }
}
