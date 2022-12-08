use error::CustomError;
use image::ImageBuffer;
use image::{Rgb, RgbImage};
use std::error::Error;

mod error;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub color: Rgb<u8>,
    pub path: &'static str,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub content: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    config: Option<Config>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            config: None,
            content: None,
        }
    }
    pub fn config(&mut self, config: Config) -> &mut Buffer {
        self.config = Some(config);
        self
    }
    pub fn build(&mut self) -> Result<Buffer, Box<dyn Error>> {
        if let Some(config) = self.config {
            let width = config.width;
            let height = config.height;
            let color = config.color;

            let mut image_buffer = RgbImage::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    image_buffer.put_pixel(x, y, color);
                }
            }
            self.content = Some(image_buffer);
            Ok(self.to_owned())
        } else {
            Err(Box::new(CustomError::BuildError))
        }
    }
    pub fn draw_rect(&mut self, x1: u32, y1: u32, w: u32, h: u32, color: Rgb<u8>) {
        if let Some(config) = self.config {
            let x2 = x1 + w;
            let y2 = y1 + h;
            if y2 > config.height || x2 > config.height {
                eprintln!("ERROR: out of canvas");
                std::process::exit(1);
            }

            for y in y1..y2 {
                for x in x1..x2 {
                    let content = self.content.as_mut().unwrap();
                    content.put_pixel(x, y, color);
                }
            }
        }
    }
    pub fn draw_circle(&mut self, cx: u32, cy: u32, r: u32, color: Rgb<u8>) {
        if let Some(config) = self.config {
            let x1 = cx - r;
            let x2 = cx + r;
            let y1 = cy - r;
            let y2 = cy + r;

            if y2 > config.height || x2 > config.height {
                eprintln!("ERROR: out of canvas");
                std::process::exit(1);
            }

            for y in y1..y2 {
                for x in x1..x2 {
                    let content = self.content.as_mut().unwrap();
                    let dx: i32 = x as i32 - cx as i32;
                    let dy: i32 = y as i32 - cy as i32;
                    if dx * dx + dy * dy < r as i32 * r as i32 {
                        content.put_pixel(x, y, color);
                    }
                }
            }
        }
    }
    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, w: u32, color: Rgb<u8>) {
        if let Some(config) = self.config {
            let content = self.content.as_mut().unwrap();
            let dx: f32 = x2 as f32 - x1 as f32;
            let dy: f32 = y2 as f32 - y1 as f32;

            if dx != 0.0 {
                let k: f32 = dy / dx;
                let c: f32 = y1 as f32 - (k * x1 as f32);

                if k == 0.0 {
                    for x in x1..x2 {
                        for y in y1..y1 + w {
                            content.put_pixel(x, y, color);
                        }
                    }
                } else {
                    if x1 > x2 {
                        for x in x2..x1 {
                            for x1 in x..x + w {
                                let y1 = k * x as f32 + c;
                                let y2 = k * (x + 1) as f32 + c;

                                for y in y2 as u32..y1 as u32 {
                                    if x1 < config.width && y < config.height {
                                        content.put_pixel(x1, y as u32, color);
                                    }
                                }
                            }
                        }
                    }
                    if x2 > x1 {
                        for x in x1..x2 {
                            for x1 in x..x + w {
                                let y1 = k * x as f32 + c;
                                let y2 = k * (x + 1) as f32 + c;

                                for y in y1 as u32..y2 as u32 {
                                    if x1 < config.width && y < config.height {
                                        content.put_pixel(x1, y as u32, color);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if y1 > y2 {
                    for y in y2..y1 {
                        for x in x1..x1 + w {
                            if x < config.width && y < config.height {
                                content.put_pixel(x, y, color);
                            }
                        }
                    }
                }
                if y2 > y1 {
                    for y in y1..y2 {
                        for x in x1..x1 + w {
                            if x < config.width && y < config.height {
                                content.put_pixel(x, y, color);
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Some(buffer) = self.content.clone() {
            let config = self.config.expect("ERROR: No config");
            buffer.save(config.path)?;
            Ok(())
        } else {
            Err(Box::new(CustomError::SaveError))
        }
    }
}
