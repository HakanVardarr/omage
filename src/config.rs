use image::Rgba;

/// The `Config` struct holds configuration settings for the drawing canvas.
///
/// It allows you to specify various parameters such as the width and height of the canvas,
/// background color, optional border color, file paths for the canvas image and an optional font file.
///
/// # Examples
///
/// ```
/// use omage::{Config, Rgba};
///
/// // Create a new Config instance with specified settings
/// let config = Config::new(800, 600, Rgba([255, 255, 255, 255]), Some(Rgba([0, 0, 0, 255])),
///                          "path/to/canvas/image.png", Some("path/to/font.ttf"));
/// ```
///
/// # Fields
///
/// - `width`: Width of the canvas.
/// - `height`: Height of the canvas.
/// - `color`: Background color of the canvas in Rgba format.
/// - `border`: Optional border color of the canvas in Rgba format.
/// - `path`: Path to the canvas image.
/// - `font_path`: Optional path to the font file.
///
/// # Methods
///
/// - `new`: Creates a new `Config` instance with the specified settings.
///
/// # Note
///
/// The `Config` struct is meant to be used to configure the canvas for drawing components,
/// and it provides a convenient way to customize various aspects of the canvas appearance.
///
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub color: Rgba<u8>,
    pub border: Option<Rgba<u8>>,
    pub path: &'static str,
    pub font_path: Option<&'static str>,
}

impl Config {
    /// Creates a new configuration for the drawing canvas.
    ///
    /// # Parameters
    ///
    /// - `width`: Width of the canvas.
    /// - `height`: Height of the canvas.
    /// - `color`: Background color of the canvas in Rgba format.
    /// - `border`: Optional border color of the canvas in Rgba format.
    /// - `path`: Path to the canvas image.
    /// - `font_path`: Optional path to the font file.
    ///
    /// # Returns
    ///
    /// A `Config` instance with the specified settings.
    pub fn new(
        width: u32,
        height: u32,
        color: Rgba<u8>,
        border: Option<Rgba<u8>>,
        path: &'static str,
        font_path: Option<&'static str>,
    ) -> Self {
        Config {
            width,
            height,
            color,
            border,
            path,
            font_path,
        }
    }
}
