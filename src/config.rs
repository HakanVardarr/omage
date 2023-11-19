use image::Rgba;

/// Configuration settings for the drawing canvas.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Width of the canvas.
    pub width: u32,
    /// Height of the canvas.
    pub height: u32,
    /// Background color of the canvas in Rgba format.
    pub color: Rgba<u8>,
    /// Optional border color of the canvas in Rgba format.
    pub border: Option<Rgba<u8>>,
    /// Path to the canvas image.
    pub path: &'static str,
    pub font_path: Option<&'static str>,
}

impl Config {
    /// Creates a new configuration with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the canvas.
    /// * `height` - Height of the canvas.
    /// * `color` - Background color of the canvas in Rgba format.
    /// * `border` - Optional border color of the canvas in Rgba format.
    /// * `path` - Path to the canvas image.
    pub fn new(
        width: u32,
        height: u32,
        color: Rgba<u8>,
        border: Option<Rgba<u8>>,
        path: &'static str,
        font_path: Option<&'static str>,
    ) -> Self {
        Self {
            width,
            height,
            color,
            border,
            path,
            font_path,
        }
    }
}
