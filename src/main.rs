#![allow(unused)]

use buffer::Buffer;
use image::ImageBuffer;
use image::Rgb;
use rimage::components::{Circle, Component, Line, Rectangle};
use rimage::config::Config;
use std::error::Error;

mod buffer;

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

const CONFIG: Config = Config {
    width: WIDTH,
    height: HEIGHT,
    color: Rgb([255, 255, 255]),
    path: "output.png",
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut components: Vec<Box<dyn Component>> = vec![];

    let row = 10;
    let col = 10;
    let rheight = HEIGHT / row;
    let rwidth = WIDTH / col;
    let mut color;

    for y in 0..row {
        for x in 0..col {
            if ((x + y) % 2) == 0 {
                color = RED;
            } else {
                color = BLACK;
            }

            let rectangle = Rectangle::new(rheight, rwidth, rwidth * x, rheight * y, color);

            components.push(Box::new(rectangle));
        }
    }

    let mut buffer = Buffer::new();

    let image = buffer.config(CONFIG).init()?.add_components(components);
    image.draw()?;

    Ok(())
}
