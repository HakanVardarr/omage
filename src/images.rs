use crate::components::ComponentTrait;

use super::{components::Component, config::Config, error::CustomError};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::error::Error;

/// Represents an image with configurable settings and drawable components.
pub struct Image {
    config: Option<Config>,
    image_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    components: Option<Vec<Component>>,
}

impl Image {
    /// Creates a new Image instance with default values.
    pub fn new() -> Self {
        Self {
            config: None,
            image_buffer: None,
            components: None,
        }
    }

    /// Sets the configuration for the image.
    pub fn config(&mut self, config: Config) -> &mut Self {
        self.config = Some(config);
        self
    }

    /// Initializes the image with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns an error if no configuration is provided.
    pub fn init(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(config) = self.config.to_owned() {
            let mut image_buffer = RgbaImage::new(config.width, config.height);

            for y in 0..config.height {
                for x in 0..config.width {
                    image_buffer.put_pixel(x, y, config.color);
                }
            }

            if let Some(border_color) = config.border {
                for x in 0..config.width {
                    image_buffer.put_pixel(x, 0, border_color);
                    image_buffer.put_pixel(x, config.height - 1, border_color)
                }
                for y in 0..config.height {
                    image_buffer.put_pixel(0, y, border_color);
                    image_buffer.put_pixel(config.width - 1, y, border_color)
                }
            }

            self.image_buffer = Some(image_buffer);
            Ok(self)
        } else {
            Err(Box::new(CustomError::NoConfigProvided))
        }
    }

    /// Adds a single component to the image.
    pub fn add_component(&mut self, component: Component) -> &mut Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }

        self
    }

    /// Adds multiple components to the image.
    pub fn add_components(&mut self, components: Vec<Component>) -> &mut Self {
        if let Some(components_list) = &mut self.components {
            components_list.extend(components);
        } else {
            self.components = Some(components);
        }

        self
    }

    /// Draws all the components on the image and saves it to the specified path.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no configuration, no components, or an issue occurs during drawing or saving.
    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
        if let Some(components) = &self.components {
            if let Some(config) = &self.config {
                if let Some(buffer) = self.image_buffer.to_owned().as_mut() {
                    for component in components {
                        let c = component;
                        c.draw(config.to_owned(), buffer)?;
                    }
                    buffer.save(config.path)?;
                    Ok(())
                } else {
                    Err(Box::new(CustomError::NoConfigProvided))
                }
            } else {
                Err(Box::new(CustomError::NoConfigProvided))
            }
        } else {
            Err(Box::new(CustomError::ThereIsNoComponent))
        }
    }
}
