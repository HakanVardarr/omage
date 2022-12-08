use super::{Component, Config, CustomError, Error, ImageBuffer, Rgb};

pub struct Circle {
    cx: u32,
    cy: u32,
    r: u32,
    color: Rgb<u8>,
}

impl Circle {
    pub fn new(cx: u32, cy: u32, r: u32, color: Rgb<u8>) -> Self {
        Self { cx, cy, r, color }
    }
}

impl Component for Circle {
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        let x1 = self.cx - self.r;
        let x2 = self.cx + self.r;
        let y1 = self.cy - self.r;
        let y2 = self.cy + self.r;

        if y2 > config.height || x2 > config.height {
            return Err(Box::new(CustomError::OutOfCanvas));
        }

        for y in y1..y2 {
            for x in x1..x2 {
                let dx: i32 = x as i32 - self.cx as i32;
                let dy: i32 = y as i32 - self.cy as i32;
                if dx * dx + dy * dy < self.r as i32 * self.r as i32 {
                    buffer.put_pixel(x, y, self.color);
                }
            }
        }

        Ok(())
    }
}
