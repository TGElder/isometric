mod utils;
mod selected_cell;
mod sea;
mod house;
mod terrain;

pub use self::utils::*;
pub use self::terrain::*;
pub use self::selected_cell::*;
pub use self::sea::*;
pub use self::house::*;

pub trait Drawing {
    fn draw(&self);
    fn get_z_mod(&self) -> f32;
}
