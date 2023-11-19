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
    },
}

/// A struct providing convenience methods for creating different types of components.
pub struct Components;

impl Components {
    /// Creates a new circle component.
    pub fn Circle(cx: u32, cy: u32, r: u32, color: Rgba<u8>) -> Component {
        Component::Circle { cx, cy, r, color }
    }

    /// Creates a new rectangle component.
    pub fn Rectangle(h: u32, w: u32, x: u32, y: u32, color: Rgba<u8>) -> Component {
        Component::Rectangle { h, w, x, y, color }
    }

    /// Creates a new line component.
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
    pub fn Text(x: u32, y: u32, size: u32, text: &'static str, color: Rgba<u8>) -> Component {
        Component::Text {
            x,
            y,
            size,
            text,
            color,
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
            } => {
                let text = Text::new(x, y, size, text, color);
                text.draw(config, buffer)
            }
        }
    }
}
