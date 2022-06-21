use super::hittable::*;
use super::ray::*;
use super::vec3::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Material {
    Diffuse(Color),
    Metalic(Color, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        match self {
            Self::Diffuse(albedo) => diffuse_scatter(ray, hit, albedo),
            Self::Metalic(albedo, fuzziness) => metalic_scatter(ray, hit, albedo, fuzziness),
            Self::Dielectric(refraction_index) => dielectric_scatter(ray, hit, refraction_index),
        }
    }
}

fn diffuse_scatter(_ray: &Ray, hit: &Hit, albedo: &Color) -> Option<(Color, Ray)> {
    let mut scatter_direction = Vec3::random_in_hemisphere(&hit.normal);
    if scatter_direction.nearly_zero() {
        scatter_direction = hit.normal;
    }
    Some((*albedo, Ray::new(hit.point, scatter_direction))) //FXME: try normal + random unit
}

fn metalic_scatter(ray: &Ray, hit: &Hit, albedo: &Color, fuziness: &f64) -> Option<(Color, Ray)> {
    let fuzz = f64::abs(*fuziness);
    let reflected =
        ray.direction.unit().reflect(hit.normal) + (Vec3::random_in_unit_sphere() * fuzz);
    if reflected.dot(hit.normal) > 0.0 {
        return Some((*albedo, Ray::new(hit.point, reflected)));
    }

    None
}

fn dielectric_scatter(ray: &Ray, hit: &Hit, refraction_index: &f64) -> Option<(Color, Ray)> {
    None
}
