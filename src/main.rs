#![allow(unused)]

use image::ImageBuffer;
use image::Rgb;
use rimage::{Buffer, Config};
use std::error::Error;

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

const CONFIG: Config = Config {
    width: WIDTH,
    height: HEIGHT,
    color: Rgb([0, 0, 255]),
    path: "output.png",
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut image_buffer = Buffer::new().config(CONFIG).build()?;

    let row = 8;
    let col = 8;
    let rheight = HEIGHT / row;
    let rwidth = WIDTH / col;
    let mut color: Rgb<u8>;
    let mut rcolor: Rgb<u8>;
    let mut piece: Rgb<u8>;

    for y in 0..row {
        for x in 0..col {
            if ((x + y) % 2 == 0) {
                color = Rgb([118, 150, 86]);
                rcolor = Rgb([238, 238, 210]);
                piece = Rgb([255, 255, 255]);
            } else {
                rcolor = Rgb([118, 150, 86]);
                color = Rgb([238, 238, 210]);
                piece = Rgb([30, 30, 30]);
            }
            image_buffer.draw_rect(x * rwidth, y * rheight, rwidth, rheight, color);
            if y < 2 || y > 5 {
                image_buffer.draw_circle(x * rwidth + 62, y * rheight + 62, 50, piece);
            }
        }
    }

    image_buffer.save()?;

    Ok(())
}
