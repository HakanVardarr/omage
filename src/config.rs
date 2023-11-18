use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub color: Rgb<u8>,
    pub border: Option<Rgb<u8>>,
    pub path: &'static str,
}

impl Config {
    pub fn new(
        width: u32,
        height: u32,
        color: Rgb<u8>,
        border: Option<Rgb<u8>>,
        path: &'static str,
    ) -> Self {
        Self {
            width,
            height,
            color,
            border,
            path,
        }
    }
}
