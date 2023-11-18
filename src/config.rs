use image::Rgb;

/// Configuration settings for the drawing canvas.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Width of the canvas.
    pub width: u32,
    /// Height of the canvas.
    pub height: u32,
    /// Background color of the canvas in RGB format.
    pub color: Rgb<u8>,
    /// Optional border color of the canvas in RGB format.
    pub border: Option<Rgb<u8>>,
    /// Path to the canvas image.
    pub path: &'static str,
}

impl Config {
    /// Creates a new configuration with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the canvas.
    /// * `height` - Height of the canvas.
    /// * `color` - Background color of the canvas in RGB format.
    /// * `border` - Optional border color of the canvas in RGB format.
    /// * `path` - Path to the canvas image.
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
