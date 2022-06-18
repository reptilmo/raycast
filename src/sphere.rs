use super::vec3::*;
use super::ray::*;
use super::hittable::*;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub location: Point3,
    pub radius: f64,
    pub material: Color,
}

impl Sphere {
    pub fn new(location: Point3, radius: f64, material: Color) -> Sphere {
        Sphere{location, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, minimum: f64, maximum: f64) -> Option<Hit> {

        let p = ray.origin - self.location;
        let a = ray.direction.dot(ray.direction);
        let b_over_2 = p.dot(ray.direction);
        let c = p.dot(p) - self.radius * self.radius;

        let discriminant =  b_over_2 * b_over_2 - a * c;
        if discriminant < 0.0 {
            return None
        }

        let a_reciprocol = 1.0 / a;
        let d_root = f64::sqrt(discriminant); //FIXME: sqrt

        let mut solution = (-b_over_2 - d_root) * a_reciprocol;
        if solution < minimum || solution > maximum {

            solution = (-b_over_2 + d_root) * a_reciprocol;
            if solution < maximum || solution > maximum {
                return None
            }
        }

        let hit_point = ray.at(solution);
        let mut normal = (hit_point - self.location) / self.radius;
        let front =  ray.direction.dot(normal) < 0.0;
        if !front {
            normal = -normal;
        }

        let hit =  Hit::new(hit_point, normal, solution, front);

        Some(hit)
    }

    fn material(&self) -> Color {
        self.material
    }
}