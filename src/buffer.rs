use super::*;
use image::Rgb;
use image::RgbImage;
use rimage::error::CustomError;

pub struct Buffer {
    config: Option<Config>,
    image_buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    components: Option<Vec<Box<dyn Component>>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            config: None,
            image_buffer: None,
            components: None,
        }
    }
    pub fn config(&mut self, config: Config) -> &mut Buffer {
        self.config = Some(config);
        self
    }
    pub fn init(&mut self) -> Result<&mut Buffer, Box<dyn Error>> {
        if let Some(config) = self.config {
            self.image_buffer = Some(RgbImage::new(config.width, config.height));
            if let Some(buffer) = &mut self.image_buffer {
                for y in 0..config.height {
                    for x in 0..config.width {
                        buffer.put_pixel(x, y, config.color);
                    }
                }
            }
        } else {
            return Err(Box::new(CustomError::NoConfigProvided));
        }

        Ok(self)
    }
    pub fn add_component(&mut self, component: Box<dyn Component>) -> &mut Buffer {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }

        self
    }
    pub fn add_components(&mut self, components: Vec<Box<dyn Component>>) -> &mut Buffer {
        if let Some(components_list) = &mut self.components {
            for component in components {
                components_list.push(component);
            }
        } else {
            self.components = Some(components);
        }

        self
    }
    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
        if let Some(components) = &self.components {
            if let Some(config) = self.config {
                if let Some(buffer) = self.image_buffer.to_owned().as_mut() {
                    for component in components {
                        let c = component.as_ref();
                        c.draw(config, buffer);
                    }
                    buffer.save(config.path);
                }
                return Ok(());
            } else {
                return Err(Box::new(CustomError::NoConfigProvided));
            }
        } else {
            return Err(Box::new(CustomError::ThereIsNoComponent));
        }
    }
}
