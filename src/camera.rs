use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub position: Point3,
    pub look_at: Point3,
    pub world_up: Vec3,
    pub focal_length: f64,
    pub aspec_ratio: f64,

    front: Vec3,
    screen_right: Vec3,
    screen_up: Vec3,
    screen_center: Point3,
}

impl Camera {
    pub fn new(
        position: Point3,
        look_at: Point3,
        world_up: Vec3,
        focal_length: f64,
        aspec_ratio: f64,
    ) -> Camera {
        let front = (look_at - position).unit();
        let mut screen_right = front.cross(world_up).unit();
        let mut screen_up = screen_right.cross(front).unit();
        let screen_center = position + (front * focal_length);

        //  screen_right = screen_right * 2.0;
        //  screen_up = screen_up * 2.0;

        //FIXME: Use apect_ratio

        Camera {
            position,
            look_at,
            world_up,
            focal_length,
            aspec_ratio,
            front,
            screen_right,
            screen_up,
            screen_center,
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.screen_center + (self.screen_right * u) + (self.screen_up * v);

        Ray::new(self.position, direction - self.position)
    }
}
