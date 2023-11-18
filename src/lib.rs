//! # omage
//!
//! `omage` is a Rust crate for creating and manipulating images with various components.
/// Colors module providing constants for common RGB colors in the `image` crate's `Rgb` format.
pub mod colors;
mod components;
mod config;
mod error;
mod images;

pub use components::Components;
pub use config::Config;
pub use image::Rgba;
pub use images::Image;
