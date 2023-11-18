use super::{ComponentTrait, Config, CustomError, Error, ImageBuffer, Rgb};

/// Represents a rectangle component with a specified height (`h`), width (`w`), position (`(x, y)`), and color.
#[derive(Clone, Copy)]
pub struct Rectangle {
    /// Height of the rectangle.
    h: u32,
    /// Width of the rectangle.
    w: u32,
    /// X-coordinate of the top-left corner of the rectangle.
    x: u32,
    /// Y-coordinate of the top-left corner of the rectangle.
    y: u32,
    /// Color of the rectangle in RGB format.
    color: Rgb<u8>,
}

impl Rectangle {
    /// Creates a new rectangle with the specified parameters.
    pub fn new(h: u32, w: u32, x: u32, y: u32, color: Rgb<u8>) -> Self {
        Self { h, w, x, y, color }
    }
}

impl ComponentTrait for Rectangle {
    /// Draws the rectangle on the provided image buffer using the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the drawing canvas.
    /// * `buffer` - Image buffer to draw the rectangle on.
    ///
    /// # Errors
    ///
    /// Returns an error if the rectangle goes beyond the canvas boundaries.
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        for x in self.x..self.x + self.w {
            if x > config.width {
                return Err(Box::new(CustomError::OutOfCanvas));
            }
            for y in self.y..self.y + self.h {
                if y > config.height {
                    return Err(Box::new(CustomError::OutOfCanvas));
                }
                buffer.put_pixel(x, y, self.color);
            }
        }

        Ok(())
    }
}
