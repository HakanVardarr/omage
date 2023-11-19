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
    /// Border (Color (Rgba<u8>), Border size).
    border: Option<(Rgba<u8>, u32)>,
}

impl Text {
    /// Creates a new text component with the specified parameters.
    pub fn new(
        x: u32,
        y: u32,
        size: u32,
        text: &'static str,
        color: Rgba<u8>,
        border: Option<(Rgba<u8>, u32)>,
    ) -> Self {
        Self {
            x,
            y,
            size,
            text,
            color,
            border,
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

            if self.border.is_some() {
                for i in 0..self.border.unwrap().1 {
                    render_text(
                        buffer,
                        &font,
                        scale,
                        self.border.unwrap().0,
                        self.text,
                        (self.x + i, self.y),
                    );
                    render_text(
                        buffer,
                        &font,
                        scale,
                        self.border.unwrap().0,
                        self.text,
                        (self.x - i, self.y),
                    );
                    render_text(
                        buffer,
                        &font,
                        scale,
                        self.border.unwrap().0,
                        self.text,
                        (self.x, self.y + i),
                    );
                    render_text(
                        buffer,
                        &font,
                        scale,
                        self.border.unwrap().0,
                        self.text,
                        (self.x, self.y - i),
                    );
                }
            }
            render_text(
                buffer,
                &font,
                scale,
                self.color,
                self.text,
                (self.x, self.y),
            );
        } else {
            return Err(Box::new(CustomError::NoFontProvided));
        }

        Ok(())
    }
}

fn render_text(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font<'static>,
    scale: Scale,
    color: Rgba<u8>,
    text: &str,
    position: (u32, u32),
) {
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(
            text,
            scale,
            point(position.0 as f32, position.1 as f32 + v_metrics.ascent),
        )
        .collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let color = Rgba([color[0], color[1], color[2], (v * color[3] as f32) as u8]);

                if x + (bounding_box.min.x as u32) < img.width()
                    && y + (bounding_box.min.y as u32) < img.height()
                {
                    img.get_pixel_mut(x + bounding_box.min.x as u32, y + bounding_box.min.y as u32)
                        .blend(&color);
                }
            })
        }
    }
}
