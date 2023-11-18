#![allow(unused, non_snake_case)]

use crate::config::Config;
use crate::error::CustomError;
use circle::Circle;
use image::{ImageBuffer, Rgb, RgbImage};
use line::Line;
use rectangle::Rectangle;
use std::error::Error;

mod circle;
mod line;
mod rectangle;

pub trait ComponentTrait {
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>>;
}

pub enum Component {
    Circle {
        cx: u32,
        cy: u32,
        r: u32,
        color: Rgb<u8>,
    },
    Rectangle {
        h: u32,
        w: u32,
        x: u32,
        y: u32,
        color: Rgb<u8>,
    },
    Line {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        color: Rgb<u8>,
    },
}

pub struct Components;

impl Components {
    pub fn Circle(cx: u32, cy: u32, r: u32, color: Rgb<u8>) -> Component {
        Component::Circle { cx, cy, r, color }
    }
    pub fn Rectangle(h: u32, w: u32, x: u32, y: u32, color: Rgb<u8>) -> Component {
        Component::Rectangle { h, w, x, y, color }
    }
    pub fn Line(x1: u32, y1: u32, x2: u32, y2: u32, color: Rgb<u8>) -> Component {
        Component::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }
}

impl ComponentTrait for Component {
    fn draw(
        &self,
        config: Config,
        buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        match *self {
            Component::Circle { cx, cy, r, color } => {
                let circle = Circle::new(cx, cy, r, color);
                circle.draw(config, buffer)
            }
            Component::Rectangle { h, w, x, y, color } => {
                let rectangle = Rectangle::new(h, w, x, y, color);
                rectangle.draw(config, buffer)
            }
            Component::Line {
                x1,
                y1,
                x2,
                y2,
                color,
            } => {
                let line = Line::new(x1, y1, x2, y2, color);
                line.draw(config, buffer)
            }
        }
    }
}
