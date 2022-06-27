use std::fs::File;
use std::str::FromStr;

extern crate image;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;

pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn clamp(lo: f64, hi: f64, value: f64) -> f64 {
    f64::max(lo, f64::min(hi, value))
}

fn pixels_to_bytes(pixels: &[Pixel]) -> &[u8] {
    let p: *const u8 = pixels.as_ptr() as *const u8;
    let n: usize = 3 * pixels.len();
    unsafe { std::slice::from_raw_parts(p, n) }
}

pub fn write_image(filename: &str, pixels: &[Pixel], bounds: (usize, usize)) {
    let file = File::create(filename).unwrap();
    let encoder = PngEncoder::new(file);
    let bytes = pixels_to_bytes(pixels);

    encoder
        .write_image(&bytes, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8)
        .expect("Failed to write image");
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<f32>("", ','), None);
    assert_eq!(parse_pair::<f32>("1.2", ','), None);
    assert_eq!(parse_pair::<f32>(",", ','), None);
    assert_eq!(parse_pair::<f32>(",3.7", ','), None);
    assert_eq!(parse_pair::<i32>("400x800", 'x'), Some((400, 800)));
    assert_eq!(
        parse_pair::<f64>("1.24,-0.6048", ','),
        Some((1.24, -0.6048))
    );
    assert_eq!(parse_pair::<f32>("3.14,25.1", ','), Some((3.14, 25.1)));
}
