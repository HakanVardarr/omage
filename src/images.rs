use crate::components::ComponentTrait;

use super::{components::Component, config::Config, error::CustomError};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::error::Error;

/// Represents an image with configurable settings and drawable components.
///
/// The `Image` struct combines configuration settings, an image buffer, and a collection of drawable
/// components to represent a 2D canvas. It allows for easy configuration and drawing of various graphical
/// elements on the canvas, such as circles, rectangles, lines, and text.
///
/// # Examples
///
/// ```
/// use omage::{Image, Config, Components, Rgba};
///
/// // Create a new image with a specified configuration
/// let config = Config::new(800, 600, Rgba([255, 255, 255, 255]), Some(Rgba([0, 0, 0, 255])), "path/to/canvas/image.png", Some("path/to/font.ttf"));
///
/// let image = Image::new_with_config(config);
///
/// // Add drawable components to the image
/// let components = vec![
///     Components::Circle(50, 50, 30, Rgba([255, 0, 0, 255])),
///     Components::Rectangle(40, 60, 10, 20, Rgba([0, 255, 0, 255])),
///     Components::Line(10, 10, 80, 80, Rgba([0, 0, 255, 255])),
///     Components::Text(30, 40, 16, "Hello, Rust!", Rgba([255, 255, 255, 255]), Some((Rgba([0, 0, 0, 255]), 2))),
/// ];
///
/// image.add_components(components);
/// ```
///
/// # Fields
///
/// - `config`: Optional configuration settings for the image canvas.
/// - `image_buffer`: Optional image buffer containing pixel data.
/// - `components`: Optional collection of drawable components to be rendered on the image.
///
/// # Methods
///
/// - `new`: Creates a new `Image` instance with default settings or a specified configuration.
/// - `config`: Adds config to `Image`
/// - `init` : Initializes the `Image`
/// - `add_component`: Adds a single drawable component to the image.
/// - `add_components`: Adds a collection of drawable components to the image.
/// - `draw`: Draws the configured image with its drawable components.
///
/// # Note
///
/// The `Image` struct provides a convenient way to configure and visualize graphical components
/// on a canvas. It encapsulates the necessary logic for image creation and component rendering.
///
/// ```
pub struct Image<'a> {
    config: Option<Config>,
    image_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    components: Option<Vec<&'a Component>>,
}

impl<'a> Image<'a> {
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
    pub fn add_component(&mut self, component: &'a Component) -> &mut Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }

        self
    }

    /// Adds multiple components to the image.
    pub fn add_components(&mut self, components: Vec<&'a Component>) -> &mut Self {
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
