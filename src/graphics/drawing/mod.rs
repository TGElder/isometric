mod house;
mod sea;
mod selected_cell;
mod terrain;
mod text;
mod billboard;
mod utils;

pub use self::house::*;
pub use self::sea::*;
pub use self::selected_cell::*;
pub use self::terrain::*;
pub use self::text::*;
pub use self::billboard::*;
pub use self::utils::*;

use ::coords::WorldCoord;
use super::engine::DrawingType;

pub trait Drawing {
    fn draw(&self);
    fn get_z_mod(&self) -> f32;
    fn drawing_type(&self) -> &DrawingType;
    fn get_visibility_check_coord(&self) -> Option<&WorldCoord>;
}
