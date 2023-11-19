//! # omage
//!
//! `omage` is a Rust crate for creating  images with various components.
//!
//! ## Features
//!
//! - **Configurable Canvas**: Easily create customizable canvases with specified dimensions, background color, and optional borders.
//! - **Draw Components**: Utilize a variety of drawable components, including circles, rectangles, lines, and text, to create complex images.
//! - **Convenient Color Constants**: Access a set of common RGBA colors through the `colors` module.
//!
//! ## Usage
//!
//! To use `omage`, you can start by creating a `Config` for your canvas, adding drawable components with the `Components` struct,
//! and rendering the image with the `Image` struct.
//!
//! ```rust
//! use omage::{Config, Components, Image, Rgba};
//!
//! // Create a new canvas configuration
//! let config = Config::new(800, 600, Rgba([255, 255, 255, 255]), Some(Rgba([0, 0, 0, 255])),
//!                          "path/to/canvas/image.png", Some("path/to/font.ttf"));
//!
//! // Creane new drawable components
//! let components = vec![
//!     Components::Circle(50, 50, 30, Rgba([255, 0, 0, 255])),
//!     Components::Rectangle(40, 60, 10, 20, Rgba([0, 255, 0, 255])),
//!     Components::Line(10, 10, 80, 80, Rgba([0, 0, 255, 255])),
//!     Components::Text(30, 40, 16, "Hello, Rust!", Rgba([255, 255, 255, 255]), Some((Rgba([0, 0, 0, 255]), 2))),
//! ];
//!
//! // Create a new image with the specified configuration
//! let mut image = Image::new();
//! image.config(config).init().unwrap().add_components(components).draw().unwrap();
//! ```
//!
//! ## Modules
//!
//! - `colors`: Constants for common RGBA colors.
//! - `components`: Convenience methods for creating different types of drawable components.
//! - `config`: Configuration settings for the drawing canvas.
//! - `error`: Custom error types for the `omage` crate.
//! - `images`: Represents an image with configurable settings and drawable components.
//!
//! ## Example
//!
//! ```rust
//! use omage::colors::*;
//! use omage::{Components, Config, Image, Rgba};
//!
//! const HEIGHT: u32 = 100;
//! const WIDTH: u32 = 300;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::new(
//!         WIDTH,
//!         HEIGHT,
//!         Rgba([255, 255, 255, 0]),
//!         Some(WHITE),
//!         "output.png",
//!         Some("./fonts/Roboto-Medium.ttf"),
//!     );
//!
//!     let mut image = Image::new();
//!
//!     let circle1 = Components::Circle(50, 55, 30, Rgba([255, 0, 0, 200]));
//!     let circle2 = Components::Circle(75, 55, 30, Rgba([0, 255, 0, 200]));
//!     let circle3 = Components::Circle(65, 35, 30, Rgba([0, 0, 255, 200]));
//!
//!     let text = "OMAGE";
//!     let text = Components::Text(
//!         config.width / 2 - 40,
//!         config.height / 2 - 25,
//!         50,
//!         text,
//!         Rgba([255, 255, 255, 255]),
//!         Some((BLACK, 3)),
//!     );
//!
//!     image
//!         .config(config)
//!         .init()?
//!         .add_components(vec![&text, &circle1, &circle2, &circle3])
//!         .draw()?;
//!     Ok(())
//! }

//! ```
//!
//! ## License
//!
//! This crate is licensed under the MIT License - see the [LICENSE](https://opensource.org/licenses/MIT) file for details.
//!

/// Constants for common RGBA colors.
pub mod colors;
mod components;
mod config;
mod error;
mod images;

pub use components::Components;
pub use config::Config;
pub use image::Rgba;
pub use images::Image;
