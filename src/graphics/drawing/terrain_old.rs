use super::super::engine::Color;
use std::f32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Junction {
    pub position: na::Vector2<usize>,
    pub width: f32,
    pub height: f32,
    color: Color,
}

impl Junction {
    pub fn new(position: na::Vector2<usize>, width: f32, height: f32, color: Color) -> Junction {
        Junction{position, width, height, color}
    }
}

#[derive(Debug, PartialEq)]
pub struct River {
    pub from: na::Vector2<usize>,
    pub to: na::Vector2<usize>,
    color: Color,
}

impl River {
    pub fn new(from: na::Vector2<usize>, to: na::Vector2<usize>, color: Color) -> River {
        if from.x < to.x || from.y < to.y {
            River{from, to, color}
        } else {
            River{from: to, to: from, color}
        }
    
    }
}