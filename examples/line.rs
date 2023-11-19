use omage::colors::*;
use omage::{Components, Config, Image, Rgba};

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        WIDTH,
        HEIGHT,
        BLACK,
        Some(GREEN),
        "output.png",
        Some("./fonts/Roboto-Medium.ttf"),
    );

    let mut image = Image::new();

    let line1 = Components::Line(0, 0, WIDTH, HEIGHT, GREEN);
    let line2 = Components::Line(WIDTH, 0, 0, HEIGHT, GREEN);
    let circle = Components::Circle(WIDTH / 2, HEIGHT / 2, 100, Rgba([0, 255, 0, 150]));
    let text = Components::Text(
        WIDTH / 2 - 210,
        HEIGHT / 2 - 250,
        40,
        "Xiaolin Wu's Line Algorithm",
        BLACK,
        Some((GREEN, 3)),
    );

    image
        .config(config)
        .init()?
        .add_components(vec![&line1, &line2, &circle, &text])
        .draw()?;
    Ok(())
}
