use super::{ComponentTrait, Config, CustomError, Error, ImageBuffer, Rgba};
use image::Pixel;

/// Represents a circle component with a specified center (`cx`, `cy`), radius (`r`), and color.
#[derive(Clone, Copy)]
pub struct Circle {
    /// X-coordinate of the circle's center.
    cx: u32,
    /// Y-coordinate of the circle's center.
    cy: u32,
    /// Radius of the circle.
    r: u32,
    /// Color of the circle in Rgba format.
    color: Rgba<u8>,
}

impl Circle {
    /// Creates a new circle with the specified parameters.
    pub fn new(cx: u32, cy: u32, r: u32, color: Rgba<u8>) -> Self {
        Self { cx, cy, r, color }
    }
}

impl ComponentTrait for Circle {
    /// Draws the circle on the provided image buffer using the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the drawing canvas.
    /// * `buffer` - Image buffer to draw the circle on.
    ///
    /// # Errors
    ///
    /// Returns an error if the circle goes beyond the canvas boundaries.
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        let x1 = self.cx as i32 - self.r as i32;
        let x2 = self.cx as i32 + self.r as i32 + 1;
        let y1 = self.cy as i32 - self.r as i32;
        let y2 = self.cy as i32 + self.r as i32 + 1;

        if y2 > config.height as i32 || x2 > config.width as i32 || x1 < 0 || y1 < 0 {
            return Err(Box::new(CustomError::OutOfCanvas));
        }

        let res = 4;
        let pad = 1.0 / (res as f32 + 1.0);
        for y in y1..y2 {
            for x in x1..x2 {
                let mut count = 0;
                // Divide Subpixels & Anti-Aliase
                for ax in 0..res {
                    for ay in 0..res {
                        let sx = x as f32 + pad * (1 + ax) as f32;
                        let sy = y as f32 + pad * (1 + ay) as f32;
                        let dx = sx - (self.cx as f32 + 0.5);
                        let dy = sy - (self.cy as f32 + 0.5);
                        if dx * dx + dy * dy <= self.r as f32 * self.r as f32 {
                            count += 1
                        }
                    }
                }
                let alpha = count as f32 / (res * res) as f32;
                let color = Rgba([
                    self.color[0],
                    self.color[1],
                    self.color[2],
                    (self.color[3] as f32 * alpha) as u8,
                ]);
                buffer.get_pixel_mut(x as u32, y as u32).blend(&color);
            }
        }

        Ok(())
    }
}
