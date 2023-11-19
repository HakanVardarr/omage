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
    fn draw(
        &self,
        _config: Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        let x0 = self.x1 as i32;
        let y0 = self.y1 as i32;
        let x1 = self.x2 as i32;
        let y1 = self.y2 as i32;

        let steep = absolute(y1 as f32 - y0 as f32) > absolute(x1 as f32 - x0 as f32);

        let (mut x0, mut y0, mut x1, mut y1) = if steep {
            (y0, x0, y1, x1)
        } else {
            (x0, y0, x1, y1)
        };

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let gradient = dy as f32 / dx as f32;

        let xend = round_number(x0 as f32);
        let yend = y0 as f32 + gradient * (xend as f32 - x0 as f32);
        let xgap = rfpart(x0 as f32 + 0.5);
        let xpxl1 = xend; // this will be used in the main loop
        let ypxl1 = ipart(yend);
        draw_pixel(buffer, xpxl1, ypxl1, rfpart(yend) * xgap, &self.color);
        draw_pixel(buffer, xpxl1, ypxl1 + 1, fpart(yend) * xgap, &self.color);
        let mut intery = yend + gradient; // first y-intersection for the main loop

        // handle second endpoint
        let xend = round_number(x1 as f32);
        let yend = y1 as f32 + gradient * (xend as f32 - x1 as f32);
        let xgap = fpart(x1 as f32 + 0.5);
        let xpxl2 = xend; // this will be used in the main loop
        let ypxl2 = ipart(yend);
        draw_pixel(buffer, xpxl2, ypxl2, rfpart(yend) * xgap, &self.color);
        draw_pixel(buffer, xpxl2, ypxl2 + 1, fpart(yend) * xgap, &self.color);

        // main loop
        if steep {
            for x in xpxl1 + 1..xpxl2 {
                draw_pixel(buffer, ipart(intery), x, rfpart(intery), &self.color);
                draw_pixel(buffer, ipart(intery) + 1, x, fpart(intery), &self.color);
                intery += gradient;
            }
        } else {
            for x in xpxl1 + 1..xpxl2 {
                draw_pixel(buffer, x, ipart(intery), rfpart(intery), &self.color);
                draw_pixel(buffer, x, ipart(intery) + 1, fpart(intery), &self.color);
                intery += gradient;
            }
        }

        Ok(())
    }
}

fn draw_pixel(
    buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    brightness: f32,
    color: &Rgba<u8>,
) {
    let new_color = Rgba([
        color[0],
        color[1],
        color[2],
        (color[3] as f32 * brightness) as u8,
    ]);

    if x < buffer.width() && y < buffer.height() {
        buffer.get_pixel_mut(x, y).blend(&new_color);
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn absolute(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

fn ipart(x: f32) -> u32 {
    x as u32
}

fn round_number(x: f32) -> u32 {
    x as u32
}

fn fpart(x: f32) -> f32 {
    if x > 0.0 {
        x - ipart(x) as f32
    } else {
        x - (ipart(x) + 1) as f32
    }
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}
