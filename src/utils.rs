use std::str::FromStr;

pub fn parse_pair<T :FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _=> None
            }
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn clamp(lo: f64, hi: f64, value: f64) -> f64 { 
    f64::max(lo, f64::min(hi, value))
} 

pub fn pixels_to_bytes(pixels: &[Pixel]) -> &[u8] {
    let p: *const u8 = pixels.as_ptr() as *const u8;
    let n: usize = 4 * pixels.len();
    unsafe { std::slice::from_raw_parts(p, n) }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<f32>("", ','), None);
    assert_eq!(parse_pair::<f32>("1.2", ','), None);
    assert_eq!(parse_pair::<f32>(",", ','), None);
    assert_eq!(parse_pair::<f32>(",3.7", ','), None);
    assert_eq!(parse_pair::<i32>("400x800", 'x'), Some((400, 800)));
    assert_eq!(parse_pair::<f64>("1.24,-0.6048", ','), Some((1.24, -0.6048)));
    assert_eq!(parse_pair::<f32>("3.14,25.1", ','), Some((3.14, 25.1)));
}
