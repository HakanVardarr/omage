use omage::colors::*;
use omage::{Components, Config, Image, Rgba};

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(WIDTH, HEIGHT, WHITE, Some(BLACK), "output.png", None);

    let mut image = Image::new();

    let circle1 = Components::Circle(config.width / 2, config.height / 2, 350, RED);
    let circle2 = Components::Circle(
        config.width / 2,
        config.height / 2,
        300,
        Rgba([255, 0, 255, 120]),
    );
    let rectangle = Components::Rectangle(
        100,
        100,
        config.width / 2 - 50,
        config.height / 2 - 50,
        Rgba([120, 0, 255, 19]),
    );

    image
        .config(config)
        .init()?
        .add_components(vec![&circle1, &circle2, &rectangle])
        .draw()?;
    Ok(())
}
