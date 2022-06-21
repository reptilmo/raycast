use super::vec3::*;
use super::ray::*;
use super::hittable::*;
use super::material::*;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub location: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(location: Point3, radius: f64, material: Material) -> Sphere {
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

        let point = ray.at(solution);
        let normal = (point - self.location) / self.radius; 
        let mut hit = Hit::new(point, normal, solution, ray.direction.dot(normal) < 0.0, self.material);

        if !hit.front {
            hit.normal = -hit.normal;
        }

        Some(hit)
    }
}