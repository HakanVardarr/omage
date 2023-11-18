#![allow(unused)]
use rimage::colors::*;
use rimage::{Components, Config, Image};

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(WIDTH, HEIGHT, WHITE, Some(BLACK), "output.png");

    let mut image = Image::new();

    let circle1 = Components::Circle(config.width / 2, config.height / 2, 300, RED);
    let circle2 = Components::Circle(config.width / 2, config.height / 2, 305, BLACK);
    let rectangle1 = Components::Rectangle(20, 20, 100, 200, PURPLE);

    image
        .config(config)
        .init()?
        .add_components(vec![circle1, circle2, rectangle1])
        .draw()?;
    Ok(())
}
