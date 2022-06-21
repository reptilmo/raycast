use super::material::*;
use super::ray::*;
use super::vec3::*;

#[derive(Clone, Debug)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub solution: f64,
    pub front: bool,
    pub material: Material,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, solution: f64, front: bool, material: Material) -> Hit {
        Hit {
            point,
            normal,
            solution,
            front,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, minimum: f64, maximum: f64) -> Option<Hit>;
}
