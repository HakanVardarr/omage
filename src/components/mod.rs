#![allow(non_snake_case)]

use crate::config::Config;
use crate::error::CustomError;
use circle::Circle;
use image::{ImageBuffer, Rgba};
use line::Line;
use rectangle::Rectangle;
use std::error::Error;
use text::Text;

mod circle;
mod line;
mod rectangle;
mod text;

/// A trait for drawing components on an image buffer.
pub trait ComponentTrait {
    /// Draws the component on the image buffer using the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the drawing canvas.
    /// * `buffer` - Image buffer to draw the component on.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue drawing the component.
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>>;
}

/// Enum representing different types of drawing components.
pub enum Component {
    /// Represents a circle component.
    Circle {
        /// X-coordinate of the circle's center.
        cx: u32,
        /// Y-coordinate of the circle's center.
        cy: u32,
        /// Radius of the circle.
        r: u32,
        /// Color of the circle in Rgba format.
        color: Rgba<u8>,
    },
    /// Represents a rectangle component.
    Rectangle {
        /// Height of the rectangle.
        h: u32,
        /// Width of the rectangle.
        w: u32,
        /// X-coordinate of the top-left corner of the rectangle.
        x: u32,
        /// Y-coordinate of the top-left corner of the rectangle.
        y: u32,
        /// Color of the rectangle in Rgba format.
        color: Rgba<u8>,
    },
    /// Represents a line component.
    Line {
        /// X-coordinate of the starting point of the line.
        x1: u32,
        /// Y-coordinate of the starting point of the line.
        y1: u32,
        /// X-coordinate of the ending point of the line.
        x2: u32,
        /// Y-coordinate of the ending point of the line.
        y2: u32,
        /// Color of the line in Rgba format.
        color: Rgba<u8>,
    },
    /// Represents a text component.
    Text {
        /// X-coordinate of the top-left corner of the text.
        x: u32,
        /// Y-coordinate of the top-left corner of the text.
        y: u32,
        /// Size of the text.
        size: u32,
        /// Text field of the text.
        text: &'static str,
        /// Color of the rectangle in Rgba format.
        color: Rgba<u8>,
        /// Border.
        border: Option<(Rgba<u8>, u32)>,
    },
}

/// A struct providing convenience methods for creating different types of components.
///
/// The `Components` struct serves as a utility for easily generating instances of various graphical
/// components in a 2D space. It offers methods for creating circles, rectangles, lines, and text
/// components with specified attributes.
///
/// # Examples
///
/// ```
/// use omage::{Components, Rgba};
///
/// // Create a new circle component
/// let circle = Components::Circle(50, 50, 30, Rgba([255, 0, 0, 255]));
///
/// // Create a new rectangle component
/// let rectangle = Components::Rectangle(40, 60, 10, 20, Rgba([0, 255, 0, 255]));
///
/// // Create a new line component
/// let line = Components::Line(10, 10, 80, 80, Rgba([0, 0, 255, 255]));
///
/// // Create a new text component
/// let text = Components::Text(30, 40, 16, "Hello, Rust!", Rgba([255, 255, 255, 255]), Some((Rgba([0, 0, 0, 255]), 2)));
/// ```
///
/// # Methods
///
/// The `Components` struct provides the following methods:
///
/// - `Circle`: Creates a new circle component with specified attributes.
/// - `Rectangle`: Creates a new rectangle component with specified attributes.
/// - `Line`: Creates a new line component with specified attributes.
/// - `Text`: Creates a new text component with specified attributes, including an optional border.
///
/// # Note
///
/// The `Components` struct is designed to simplify the process of creating graphical components
/// for use in a 2D drawing context. It encapsulates the logic for constructing different types
/// of components with ease.
/// ```
pub struct Components;

impl Components {
    /// Creates a new circle component.
    ///
    /// # Parameters
    ///
    /// - `cx`: X-coordinate of the center of the circle.
    /// - `cy`: Y-coordinate of the center of the circle.
    /// - `r`: Radius of the circle.
    /// - `color`: RGBA color of the circle.
    ///
    /// # Returns
    ///
    /// A `Component::Circle` instance.
    pub fn Circle(cx: u32, cy: u32, r: u32, color: Rgba<u8>) -> Component {
        Component::Circle { cx, cy, r, color }
    }

    /// Creates a new rectangle component.
    ///
    /// # Parameters
    ///
    /// - `h`: Height of the rectangle.
    /// - `w`: Width of the rectangle.
    /// - `x`: X-coordinate of the top-left corner of the rectangle.
    /// - `y`: Y-coordinate of the top-left corner of the rectangle.
    /// - `color`: RGBA color of the rectangle.
    ///
    /// # Returns
    ///
    /// A `Component::Rectangle` instance.
    pub fn Rectangle(h: u32, w: u32, x: u32, y: u32, color: Rgba<u8>) -> Component {
        Component::Rectangle { h, w, x, y, color }
    }

    /// Creates a new line component.
    ///
    /// # Parameters
    ///
    /// - `x1`: X-coordinate of the starting point of the line.
    /// - `y1`: Y-coordinate of the starting point of the line.
    /// - `x2`: X-coordinate of the ending point of the line.
    /// - `y2`: Y-coordinate of the ending point of the line.
    /// - `color`: RGBA color of the line.
    ///
    /// # Returns
    ///
    /// A `Component::Line` instance.
    pub fn Line(x1: u32, y1: u32, x2: u32, y2: u32, color: Rgba<u8>) -> Component {
        Component::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }

    /// Creates a new text component.
    ///
    /// # Parameters
    ///
    /// - `x`: X-coordinate of the text.
    /// - `y`: Y-coordinate of the text.
    /// - `size`: Font size of the text.
    /// - `text`: The actual text content.
    /// - `color`: RGBA color of the text.
    /// - `border`: Optional border color and thickness as a tuple.
    ///
    /// # Returns
    ///
    /// A `Component::Text` instance.
    pub fn Text(
        x: u32,
        y: u32,
        size: u32,
        text: &'static str,
        color: Rgba<u8>,
        border: Option<(Rgba<u8>, u32)>,
    ) -> Component {
        Component::Text {
            x,
            y,
            size,
            text,
            color,
            border,
        }
    }
}

impl ComponentTrait for Component {
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        match *self {
            Component::Circle { cx, cy, r, color } => {
                let circle = Circle::new(cx, cy, r, color);
                circle.draw(config, buffer)
            }
            Component::Rectangle { h, w, x, y, color } => {
                let rectangle = Rectangle::new(h, w, x, y, color);
                rectangle.draw(config, buffer)
            }
            Component::Line {
                x1,
                y1,
                x2,
                y2,
                color,
            } => {
                let line = Line::new(x1, y1, x2, y2, color);
                line.draw(config, buffer)
            }
            Component::Text {
                x,
                y,
                size,
                text,
                color,
                border,
            } => {
                let text = Text::new(x, y, size, text, color, border);
                text.draw(config, buffer)
            }
        }
    }
}
