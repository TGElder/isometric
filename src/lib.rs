mod engine;
pub mod events;
pub mod event_handlers;
mod graphics;
mod utils;
pub mod terrain;
pub mod coords;
mod transform;
mod color;

pub use engine::*;
pub use color::Color;
pub use graphics::drawing;
pub use graphics::texture::*;

pub extern crate nalgebra as na;
pub extern crate glutin;
pub extern crate image;

use std::fmt::Debug;

pub type M<T> = na::DMatrix<T>;
pub type V2<T> = na::Vector2<T>;
pub type V3<T> = na::Vector3<T>;

fn v2 <T: 'static + Copy + PartialEq + Debug> (x: T, y: T) -> na::Vector2<T> {
    na::Vector2::new(x, y)
}

fn v3 <T: 'static + Copy + PartialEq + Debug> (x: T, y: T, z: T) -> na::Vector3<T> {
    na::Vector3::new(x, y, z)
}