mod utils;
mod selected_cell;
mod sea;
mod house;
mod terrain;
mod canvas;
mod text;

pub use self::utils::*;
pub use self::terrain::*;
pub use self::selected_cell::*;
pub use self::sea::*;
pub use self::house::*;
pub use self::canvas::*;
pub use self::text::*;

use super::engine::DrawingType;
use super::vertex_objects::VBO;

pub trait Drawing {
    fn draw(&self);
    fn get_z_mod(&self) -> f32;
    fn drawing_type(&self) -> &DrawingType;
}