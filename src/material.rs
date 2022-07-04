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
    Some((*albedo, Ray::new(hit.point, scatter_direction)))
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
    let refraction_ratio = match hit.front {
        true => 1.0 / *refraction_index,
        false => *refraction_index,
    };

    let unit_direction = ray.direction.unit();

    // Snell's Law
    let cos_theta = f64::min(-unit_direction.dot(hit.normal), 1.0);
    let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

    let scatter_direction = match refraction_ratio * sin_theta > 1.0 {
        true => unit_direction.reflect(hit.normal),
        false => {
            let perpendicular = (unit_direction + hit.normal * cos_theta) * refraction_ratio;
            let parallel = hit.normal * -f64::sqrt(f64::abs(1.0 - perpendicular.magnitude2()));
            perpendicular + parallel
        }
    };

    Some((
        Color::new(1.0, 1.0, 1.0),
        Ray::new(hit.point, scatter_direction),
    ))
}
