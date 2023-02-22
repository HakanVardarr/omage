#![allow(unused)]

use buffer::Buffer;
use image::ImageBuffer;
use image::Rgb;
use rimage::components::{Circle, Component, Line, Rectangle};
use rimage::config::Config;
use std::error::Error;

mod buffer;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const PURPLE: Rgb<u8> = Rgb([150, 0, 150]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);

const CONFIG: Config = Config {
    width: WIDTH,
    height: HEIGHT,
    color: BLACK,
    path: "output.png",
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = Buffer::new();

    let image = buffer
        .config(CONFIG)
        .init()?
        .add_component(Box::new(Circle::new(
            CONFIG.width / 2,
            CONFIG.height / 2,
            100,
            PURPLE,
        )))
        .draw()?;

    Ok(())
}
