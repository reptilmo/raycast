use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new(position: Point3, look_at: Point3, world_up: Vec3) -> Self {
        let front = (position - look_at).unit();
        let right = world_up.cross(front);
        let up = front.cross(right);

        Self {
            origin: position,
            front: front,
            right: right,
            up: up,
        }
    }

    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.origin, Vec3::new(x, y, -1.0) - self.origin)
    }
}
