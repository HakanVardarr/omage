use omage::colors::*;
use omage::{Components, Config, Image, Rgba};

const HEIGHT: u32 = 100;
const WIDTH: u32 = 300;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        WIDTH,
        HEIGHT,
        Rgba([255, 255, 255, 0]),
        None,
        "output.png",
        Some("./fonts/Roboto-Medium.ttf"),
    );

    let mut image = Image::new();

    let circle1 = Components::Circle(50, 55, 30, Rgba([255, 0, 0, 200]));
    let circle2 = Components::Circle(75, 55, 30, Rgba([0, 255, 0, 200]));
    let circle3 = Components::Circle(65, 35, 30, Rgba([0, 0, 255, 200]));

    let text = "ROMAGE";
    let text = Components::Text(
        config.width / 2 - 40,
        config.height / 2 - 25,
        50,
        text,
        WHITE,
    );

    image
        .config(config)
        .init()?
        .add_components(vec![&text, &circle1, &circle2, &circle3])
        .draw()?;
    Ok(())
}
