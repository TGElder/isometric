extern crate glutin;
use ::graphics::engine::GraphicsEngine;

trait PhysicalPositionExt {
    fn to_gl_coord_2d(self) -> GLCoord2D;
}

impl PhysicalPositionExt for glutin::dpi::PhysicalPosition {
    fn to_gl_coord_2d(self) -> GLCoord2D {
        GLCoord2D{x: 0.0, y: 0.0}
    }
}

pub struct GLCoord2D{
    pub x: f32,
    pub y: f32,
}

impl GLCoord2D {
    pub fn to_gl_coord_4d(self, graphics: &GraphicsEngine) -> GLCoord4D {
        GLCoord4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

pub struct GLCoord4D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl GLCoord4D {
    pub fn to_world_coord(self, graphics: &GraphicsEngine) -> WorldCoord {
        WorldCoord{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}


pub struct WorldCoord{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldCoord {
    pub fn to_gl_coord4d(self, graphics: &GraphicsEngine) -> GLCoord4D {
        GLCoord4D{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}
