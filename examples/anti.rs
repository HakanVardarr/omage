use omage::colors::*;
use omage::{Components, Config, Image};

const HEIGHT: u32 = 60;
const WIDTH: u32 = 90;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(WIDTH, HEIGHT, WHITE, None, "output.png", None);

    let mut image = Image::new();

    let circle = Components::Circle(config.width / 2, config.height / 2, 10, RED);

    image.config(config).init()?.add_component(&circle).draw()?;
    Ok(())
}
