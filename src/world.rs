use super::hittable::*;
use super::ray::*;
use super::sphere::*;

pub struct World {
    spheres: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            spheres: Vec::<Sphere>::new(),
        }
    }

    pub fn add_sphere(&mut self, s: Sphere) {
        self.spheres.push(s);
    }

    pub fn test_camera_ray(&self, ray: &Ray) -> Option<Hit> {
        const MINIMUM_SOLUTION: f64 = 1e-6;
        let mut maximum_solution = f64::INFINITY;
        let mut out: Option<Hit> = None;

        for object in self.spheres.iter() {
            match object.hit(ray, MINIMUM_SOLUTION, maximum_solution) {
                None => (),
                Some(hit) => {
                    maximum_solution = hit.solution;
                    out = Some(hit);
                }
            }
        }

        out
    }
}
