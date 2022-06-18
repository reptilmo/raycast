use std::ops;
use std::cmp;

pub const PI: f64 = std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    #[inline]
    pub fn nearly_eq(&self, other: Vec3) -> bool {
        f64::abs(self.x - other.x) <= f64::EPSILON * 10.0
            && f64::abs(self.y - other.y) <= f64::EPSILON * 10.0
            && f64::abs(self.z - other.z) <= f64::EPSILON * 10.0
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
            return *self * (1.0 / self.magnitude())
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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
        Vec3{x: self.x + other.x,  y: self.y + other.y, z: self.z + other.z}
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
        Vec3{x: self.x - other.x,  y: self.y - other.y, z: self.z - other.z}
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
pub type Color = Vec3; //FIXME: implement color and material

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
}
