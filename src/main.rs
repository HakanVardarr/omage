#![allow(unused)]

use image::ImageBuffer;
use image::Rgb;
use rimage::{Buffer, Config};
use std::error::Error;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 500;

const CONFIG: Config = Config {
    width: WIDTH,
    height: HEIGHT,
    color: Rgb([0, 0, 255]),
    path: "output.png",
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut image_buffer = Buffer::new().config(CONFIG).build()?;

    let row = 10;
    let col = 10;
    let rheight = HEIGHT / row;
    let rwidth = WIDTH / col;
    let mut color: Rgb<u8>;

    for y in 0..row {
        for x in 0..col {
            if ((x + y) % 2 == 0) {
                color = Rgb([255, 255, 255]);
            } else {
                color = Rgb([0, 0, 0]);
            }
            image_buffer.draw_rect(x * rwidth, y * rheight, rwidth, rheight, color)
        }
    }

    image_buffer.save()?;

    Ok(())
}
