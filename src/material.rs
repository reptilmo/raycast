use super::vec3::*;
use super::ray::*;
use super::hittable::*;

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Material  {
    Diffuse(Color),
    Metalic(Color, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color,Ray)> {
        match self {
            Self::Diffuse(albedo) => diffuse_scatter(ray, hit, albedo),
            Self::Metalic(albedo, fuzziness) => metalic_scatter(ray, hit, albedo, fuzziness),
            Self::Dielectric(refraction_index) => dielectric_scatter(ray, hit, refraction_index),
        }
    }
}

fn diffuse_scatter(ray: &Ray, hit: &Hit, albedo: &Color) ->Option<(Color,Ray)> {
    Some((*albedo, Ray::new(hit.point, Vec3::random_in_hemisphere(&hit.normal)))) //FXME: use normal + random unit 
}

fn metalic_scatter(ray: &Ray, hit: &Hit, albedo: &Color, fuziness: &f64) ->Option<(Color,Ray)> {
    None
}

fn dielectric_scatter(ray: &Ray, hit: &Hit, refraction_index: &f64) ->Option<(Color,Ray)> {
    None
}

