use super::transform::Transform;

#[derive(PartialEq, Debug)]
pub struct GLCoord2D{
    pub x: f32,
    pub y: f32,
}

impl GLCoord2D {
    pub fn new(x: f32, y: f32) -> GLCoord2D {
        GLCoord2D{x, y}
    }
}

#[derive(PartialEq, Debug)]
pub struct GLCoord3D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GLCoord3D {
    pub fn new(x: f32, y: f32, z: f32) -> GLCoord3D {
        GLCoord3D{x, y, z}
    }
    
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GLCoord4D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl GLCoord4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> GLCoord4D {
        GLCoord4D{x, y, z, w}
    }

    pub fn to_world_coord(self, transformer: &Transform) -> WorldCoord {
        transformer.unproject(self)
    }
}

impl Into<GLCoord4D> for na::Point4<f32> {
    fn into(self) -> GLCoord4D {
        GLCoord4D{
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
}

impl Into<na::Point4<f32>> for GLCoord4D {
    fn into(self) -> na::Point4<f32> {
        na::Point4::new(
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WorldCoord{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldCoord {
    pub fn new(x: f32, y: f32, z: f32) -> WorldCoord {
        WorldCoord{x, y, z}
    }

    pub fn to_gl_coord_4d(self, transformer: &Transform) -> GLCoord4D {
        transformer.project(self)
    }
}

impl Into<WorldCoord> for na::Point4<f32> {
    fn into(self) -> WorldCoord {
        WorldCoord{
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<na::Point4<f32>> for WorldCoord {
    fn into(self) -> na::Point4<f32> {
        na::Point4::new(
            self.x,
            self.y,
            self.z,
            1.0
        )
    }
}