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
        let x1 = self.cx - self.r;
        let x2 = self.cx + self.r;
        let y1 = self.cy - self.r;
        let y2 = self.cy + self.r;

        if y2 > config.height || x2 > config.width {
            return Err(Box::new(CustomError::OutOfCanvas));
        }

        for y in y1..y2 {
            for x in x1..x2 {
                let dx: i32 = x as i32 - self.cx as i32;
                let dy: i32 = y as i32 - self.cy as i32;

                if dx * dx + dy * dy < self.r as i32 * self.r as i32 {
                    buffer.get_pixel_mut(x, y).blend(&self.color);
                }
            }
        }

        Ok(())
    }
}
