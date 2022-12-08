use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub color: Rgb<u8>,
    pub path: &'static str,
}
