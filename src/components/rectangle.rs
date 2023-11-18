use super::{ComponentTrait, Config, CustomError, Error, ImageBuffer, Rgb};

#[derive(Clone, Copy)]
pub struct Rectangle {
    h: u32,
    w: u32,
    x: u32,
    y: u32,
    color: Rgb<u8>,
}

impl Rectangle {
    pub fn new(h: u32, w: u32, x: u32, y: u32, color: Rgb<u8>) -> Self {
        Self { h, w, x, y, color }
    }
}

impl ComponentTrait for Rectangle {
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
