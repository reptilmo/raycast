use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

#[derive(Clone, Debug)]
pub struct Sphere {
    location: Point3,
    radius: f64,
    material: Material,
    one_over_radius: f64,
}

impl Sphere {
    pub fn new(location: Point3, radius: f64, material: Material) -> Sphere {
        let one_over_radius = 1.0 / radius;

        Sphere {
            location,
            radius,
            material,
            one_over_radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, minimum: f64, maximum: f64) -> Option<Hit> {
        let p = ray.origin - self.location;
        let a = ray.direction.dot(ray.direction);
        let b_over_2 = p.dot(ray.direction);
        let c = p.dot(p) - self.radius * self.radius;

        let discriminant = b_over_2 * b_over_2 - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let a_reciprocol = 1.0 / a;
        let d_root = f64::sqrt(discriminant);

        let mut solution = (-b_over_2 - d_root) * a_reciprocol;
        if solution < minimum || solution > maximum {
            solution = (-b_over_2 + d_root) * a_reciprocol;
            if solution < minimum || solution > maximum {
                return None;
            }
        }

        let point = ray.at(solution);
        let normal = (point - self.location) * self.one_over_radius;
        let mut hit = Hit::new(
            point,
            normal,
            solution,
            ray.direction.dot(normal) < 0.0,
            self.material,
        );

        if !hit.front {
            hit.normal = -hit.normal;
        }

        Some(hit)
    }
}

#[test]
fn hit_sphere() {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, Material::Dielectric(1.5));

    let ray = Ray::new(Point3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);

    assert!(hit.is_some());
    let hit = hit.unwrap();

    assert!(hit.front);
    assert_eq!(hit.solution, 0.0);
    assert_eq!(hit.point, Point3::new(0.0, 0.0, 1.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(hit.material, Material::Dielectric(1.5));

    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);

    assert!(hit.is_some());
    let hit = hit.unwrap();

    assert!(!hit.front);
    assert_eq!(hit.solution, 1.0);
    assert_eq!(hit.point, Point3::new(0.0, 0.0, -1.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(hit.material, Material::Dielectric(1.5));

    let ray = Ray::new(Point3::new(0.0, 1.1, 0.0), Vec3::new(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);

    assert!(hit.is_none());
}
