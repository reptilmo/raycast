use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub position: Point3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new(position: Point3, look_at: Point3, world_up: Vec3) -> Camera {
        let front = (position - look_at).unit();
        let right = world_up.cross(front);
        let up = front.cross(right);

        Camera {
            position: position,
            front: front,
            right: right,
            up: up,
        }
    }

    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let world_dir = Vec3::new(x, y, -1.0);

        let cam_dir_x = world_dir.dot(self.right);
        let cam_dir_y = world_dir.dot(self.up);
        let cam_dir_z = world_dir.dot(-self.front);

        Ray::new(
            self.position,
            Vec3::new(cam_dir_x, cam_dir_y, cam_dir_z) - self.position,
        )
    }
}
