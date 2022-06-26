use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Point3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
    aspect_ratio: f64,
    fov_factor: f64,
}

impl Camera {
    pub fn new(
        position: Point3,
        look_at: Point3,
        world_up: Vec3,
        aspect_ratio: f64,
        fov: f64,
    ) -> Camera {
        let front = (position - look_at).unit();
        let right = world_up.cross(front);
        let up = front.cross(right);

        Camera {
            origin: position,
            front: front,
            right: right,
            up: up,
            aspect_ratio: aspect_ratio,
            fov_factor: f64::tan(fov * 0.5 * PI / 180.0),
        }
    }

    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let horizontal = self.right * x * self.aspect_ratio * self.fov_factor;
        let vertical = self.up * y * self.fov_factor;

        Ray::new(
            self.origin,
            horizontal + vertical - self.front - self.origin,
        )
    }
}
