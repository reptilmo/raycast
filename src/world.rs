use super::vec3::*;
use super::ray::*;
use super::hittable::*;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> World {
        World{objects: Vec::<Box<dyn Hittable>>::new() }
    }

    pub fn add_object(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn test_camera_ray(&self, ray: &Ray) -> Color {

        let minimum_solution = 0.0;
        let mut maximum_solution = f64::INFINITY;
        let mut material = Color::new(1.0, 1.0, 1.0);

        for object in self.objects.iter() {
            match object.hit(ray, minimum_solution, maximum_solution) {
                Some(hit) => {
                    maximum_solution = hit.solution;
                    material = hit.normal;

                },
                None => (),
            }
        }

        material
    }
}



