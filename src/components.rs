#![allow(unused)]

use crate::config::Config;
use crate::error::CustomError;
pub use circle::Circle;
use image::{ImageBuffer, Rgb, RgbImage};
pub use rectangle::Rectangle;
use std::error::Error;

mod circle;
mod rectangle;

pub trait Component {
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>>;
}
