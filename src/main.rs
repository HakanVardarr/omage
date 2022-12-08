#![allow(unused)]

use buffer::Buffer;
use image::ImageBuffer;
use image::Rgb;
use rimage::components::{Circle, Component, Rectangle};
use rimage::config::Config;
use std::error::Error;

mod buffer;

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

const CONFIG: Config = Config {
    width: WIDTH,
    height: HEIGHT,
    color: Rgb([255, 255, 255]),
    path: "output.png",
};

fn main() -> Result<(), Box<dyn Error>> {
    let rect1 = Rectangle::new(600, 600, 200, 200, Rgb([0, 255, 0]));
    let rect2 = Rectangle::new(500, 500, 250, 250, Rgb([0, 0, 255]));
    let circ = Circle::new(500, 500, 150, Rgb([0, 0, 0]));

    let image = Buffer::new()
        .config(CONFIG)
        .init()
        .add_component(Box::new(rect1))
        .add_component(Box::new(rect2))
        .add_component(Box::new(circ))
        .draw();

    Ok(())
}
