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

pub fn write_image(filename: &str, bytes: &[u8], width: usize, height: usize) {
    let file = File::create(filename).unwrap();
    let encoder = PngEncoder::new(file);

    encoder
        .write_image(&bytes, width as u32, height as u32, ColorType::Rgb8)
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
