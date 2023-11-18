use crate::components::ComponentTrait;

use super::{components::Component, config::Config, error::CustomError};
use image::{ImageBuffer, Rgb, RgbImage};
use std::error::Error;

pub struct Image {
    config: Option<Config>,
    image_buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    components: Option<Vec<Component>>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            config: None,
            image_buffer: None,
            components: None,
        }
    }
    pub fn config(&mut self, config: Config) -> &mut Self {
        self.config = Some(config);
        self
    }
    pub fn init(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(config) = self.config {
            let mut image_buffer = RgbImage::new(config.width, config.height);

            for y in 0..config.height {
                for x in 0..config.width {
                    image_buffer.put_pixel(x, y, config.color);
                }
            }

            if config.border.is_some() {
                for x in 0..config.width {
                    image_buffer.put_pixel(x, 0, config.border.unwrap());
                    image_buffer.put_pixel(x, config.height - 1, config.border.unwrap())
                }
                for y in 0..config.height {
                    image_buffer.put_pixel(0, y, config.border.unwrap());
                    image_buffer.put_pixel(config.width - 1, y, config.border.unwrap())
                }
            }

            self.image_buffer = Some(image_buffer)
        } else {
            return Err(Box::new(CustomError::NoConfigProvided));
        }

        Ok(self)
    }

    pub fn add_component(&mut self, component: Component) -> &mut Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }

        self
    }

    pub fn add_components(&mut self, components: Vec<Component>) -> &mut Self {
        if let Some(components_list) = &mut self.components {
            components_list.extend(components);
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
                        let c = component;
                        c.draw(config, buffer)?;
                    }
                    buffer.save(config.path)?;
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
