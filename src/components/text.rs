use crate::error::CustomError;

use super::ComponentTrait;
use image::{ImageBuffer, Pixel, Rgba};
use rusttype::{point, Font, Scale};
use std::io::Read;

/// Represents a text component with a specified position, size, content, and color.
pub struct Text {
    /// X-coordinate of the top-left corner of the text.
    x: u32,
    /// Y-coordinate of the top-left corner of the text.
    y: u32,
    /// Size of the text.
    size: u32,
    /// Text content of the text.
    text: &'static str,
    /// Color of the text in Rgba format.
    color: Rgba<u8>,
}

impl Text {
    /// Creates a new text component with the specified parameters.
    pub fn new(x: u32, y: u32, size: u32, text: &'static str, color: Rgba<u8>) -> Self {
        Self {
            x,
            y,
            size,
            text,
            color,
        }
    }
}

impl ComponentTrait for Text {
    /// Draws the text on the provided image buffer using the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the drawing canvas.
    /// * `buffer` - Image buffer to draw the text on.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue with the font or drawing the text.
    fn draw(
        &self,
        config: crate::Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(font_path) = config.font_path {
            let mut fonts = std::fs::File::open(font_path)?;
            let mut bytes = Vec::new();
            fonts.read_to_end(&mut bytes)?;
            let font = Font::try_from_vec(bytes).expect("Error Constructing Font");
            let scale = Scale::uniform(self.size as f32);
            let v_metrics = font.v_metrics(scale);
            let glyphs: Vec<_> = font
                .layout(self.text, scale, point(0.0, v_metrics.ascent))
                .collect();

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        let color = Rgba([
                            self.color[0],
                            self.color[1],
                            self.color[2],
                            (v * self.color[3] as f32) as u8,
                        ]);

                        buffer
                            .get_pixel_mut(
                                x + bounding_box.min.x as u32 + self.x,
                                y + bounding_box.min.y as u32 + self.y,
                            )
                            .blend(&color);
                    })
                }
            }
        } else {
            return Err(Box::new(CustomError::NoFontProvided));
        }

        Ok(())
    }
}