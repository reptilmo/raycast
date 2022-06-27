use super::hittable::*;
use super::ray::*;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::<Box<dyn Hittable>>::new(),
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn test_camera_ray(&self, ray: &Ray) -> Option<Hit> {
        let minimum_solution = 0.0001;
        let mut maximum_solution = f64::INFINITY;
        let mut out: Option<Hit> = None;

        for object in self.objects.iter() {
            match object.hit(ray, minimum_solution, maximum_solution) {
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
