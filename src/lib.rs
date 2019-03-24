pub mod engine;
mod events;
mod event_handlers;
pub mod graphics;
mod utils;
pub mod terrain;

extern crate nalgebra as na;

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