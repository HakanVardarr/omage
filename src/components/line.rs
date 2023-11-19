use super::{ComponentTrait, Config, Error, ImageBuffer, Rgba};
use image::Pixel;

/// Represents a line component with two endpoints (`(x1, y1)` and `(x2, y2)`) and a specified color.
#[derive(Clone, Copy)]
pub struct Line {
    /// X-coordinate of the first endpoint.
    x1: u32,
    /// Y-coordinate of the first endpoint.
    y1: u32,
    /// X-coordinate of the second endpoint.
    x2: u32,
    /// Y-coordinate of the second endpoint.
    y2: u32,
    /// Color of the line in Rgba format.
    color: Rgba<u8>,
}

impl Line {
    /// Creates a new line with the specified parameters.
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32, color: Rgba<u8>) -> Line {
        Line {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }
}

impl ComponentTrait for Line {
    /// Draws the line on the provided image buffer using the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the drawing canvas.
    /// * `buffer` - Image buffer to draw the line on.
    ///
    /// # Errors
    ///
    /// Returns an error if the line goes beyond the canvas boundaries.
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut x1 = self.x1.clone();
        let mut x2 = self.x2.clone();
        let mut y1 = self.y1.clone();
        let mut y2 = self.y2.clone();

        let dx: f32 = self.x2 as f32 - self.x1 as f32;
        let dy: f32 = self.y2 as f32 - self.y1 as f32;

        if dx != 0.0 {
            let k: f32 = dy / dx;
            let c: f32 = self.y1 as f32 - (k * self.x1 as f32);

            if x2 < x1 {
                x1 = self.x2;
                x2 = self.x1;
            }

            if k == 0.0 {
                for x in x1..x2 {
                    buffer.get_pixel_mut(x, y1).blend(&self.color)
                }
            } else {
                for x in x1..x2 {
                    let by1 = k * x as f32 + c;
                    let by2 = k * (x + 3) as f32 + c;

                    if by1 > by2 {
                        for y in by2 as u32..by1 as u32 {
                            if x < config.width && y < config.height {
                                buffer.get_pixel_mut(x, y).blend(&self.color)
                            }
                        }
                    }
                    if by2 > by1 {
                        for y in by1 as u32..by2 as u32 {
                            if x < config.width && y < config.height {
                                buffer.get_pixel_mut(x, y).blend(&self.color)
                            }
                        }
                    }
                }
            }
        } else {
            if y2 < y1 {
                y1 = self.y2;
                y2 = self.y1;
            }

            for y in y1..y2 {
                if x1 < config.width && y < config.height {
                    buffer.get_pixel_mut(x1, y).blend(&self.color)
                }
            }
        }

        Ok(())
    }
}
