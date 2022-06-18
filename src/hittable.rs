use super::vec3::*;
use super::ray::*;

#[derive(Clone,Debug)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub solution: f64,
    pub front: bool,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, solution: f64, front: bool) -> Hit {
        Hit{point, normal, solution, front}
    }
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, minimum: f64, maximum: f64) -> Option<Hit>;
    fn material(&self) -> Color;
}
