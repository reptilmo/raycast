use rand::prelude::*;
use std::cmp;
use std::ops;

#[allow(dead_code)]
pub const PI: f64 = std::f64::consts::PI;

pub const EPSILON: f64 = f64::EPSILON * 10.0;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random_color() -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let v = Vec3 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
                z: rng.gen_range(-1.0..1.0),
            };

            if v.magnitude2() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let random = Vec3::random_in_unit_sphere();
        if random.dot(*normal) > 0.0 {
            return random;
        }

        -random
    }

    #[inline]
    pub fn nearly_zero(&self) -> bool {
        f64::abs(self.x) <= EPSILON && f64::abs(self.y) <= EPSILON && f64::abs(self.z) <= EPSILON
    }

    #[allow(dead_code)]
    #[inline]
    pub fn nearly_eq(&self, other: Vec3) -> bool {
        f64::abs(self.x - other.x) <= EPSILON
            && f64::abs(self.y - other.y) <= EPSILON
            && f64::abs(self.z - other.z) <= EPSILON
    }

    #[inline]
    pub fn magnitude2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.magnitude2())
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        let magnitude = self.magnitude();

        if magnitude >= 0.0 {
            return *self * (1.0 / self.magnitude());
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        // dirtection (self) is pointing in, normal is pointing out
        *self - (normal * 2.0 * self.dot(normal))
    }

    #[allow(dead_code)]
    pub fn refract(&self, normal: Vec3, ratio: f64) -> Vec3 {
        // Snell's Law
        let cost_theta = f64::min(-self.dot(normal), 1.0);
        let perpendicular = (*self + normal * cost_theta) * ratio;
        let parallel = normal * -f64::sqrt(f64::abs(1.0 - perpendicular.magnitude2()));
        perpendicular + parallel
    }
}

impl cmp::PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl cmp::Eq for Vec3 {}

impl ops::Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f64) -> Vec3 {
        let factor = 1.0 / other;
        Vec3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn add() {
        let a = Vec3::new(10.0, 25.0, 3.0);
        let b = Vec3::new(12.0, 10.0, 7.0);
        let c = Vec3::new(22.0, 35.0, 10.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Vec3::new(22.0, 35.0, 10.0);
        let b = Vec3::new(12.0, 10.0, 7.0);
        let c = Vec3::new(10.0, 25.0, 3.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn nearly_equal() {
        let a = Vec3::new(1.2512, 1.5519, 1.3317);
        let b = Vec3::new(2.3701, 1.5501, 1.3313);
        let c = Vec3::new(3.6213, 3.102, 2.663);
        assert!((a + b).nearly_eq(c));
    }

    #[test]
    fn mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 10.0, 15.0);
        assert_eq!(a * 5.0, b);
    }

    #[test]
    fn div() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(a / 2.0, b);
    }

    #[test]
    fn random_in_unit_sphere() {
        //FIXME: bad unit test
        let a = Vec3::random_in_unit_sphere();
        assert!(a.magnitude() < 1.0);
    }
}
