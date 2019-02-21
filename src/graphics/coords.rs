extern crate glutin;
use super::transformer::Transformer;

pub trait PhysicalPositionExt {
    fn to_gl_coord_2d(self, physical_size: glutin::dpi::PhysicalSize) -> GLCoord2D;
    fn to_gl_coord_4d <T: ZFinder> (self, physical_size: glutin::dpi::PhysicalSize, z_finder: &T) -> GLCoord4D;
}

impl PhysicalPositionExt for glutin::dpi::PhysicalPosition {
    fn to_gl_coord_2d(self, physical_size: glutin::dpi::PhysicalSize) -> GLCoord2D {
        GLCoord2D{
            x: ( ( ( self.x / physical_size.width ) * 2.0 ) - 1.0 ) as f32,
            y: ( 1.0 - ( ( self.y / physical_size.height ) * 2.0 ) ) as f32,
        }
    }

    fn to_gl_coord_4d <T: ZFinder> (self, physical_size: glutin::dpi::PhysicalSize, z_finder: &T) -> GLCoord4D {
        let buffer_coord = glutin::dpi::PhysicalPosition::new(
            self.x,
            physical_size.height - self.y
        );
        let gl_coord_2d = self.to_gl_coord_2d(physical_size);
        GLCoord4D{
            x: gl_coord_2d.x,
            y: gl_coord_2d.y,
            z: z_finder.get_z_at(buffer_coord),
            w: 1.0
        }
    }
}

pub struct BufferCoord {
    pub x: i32,
    pub y: i32
}

pub trait ZFinder {
    fn get_z_at(&self, screen_coordinate: glutin::dpi::PhysicalPosition) -> f32;
}

#[derive(PartialEq, Debug)]
pub struct GLCoord2D{
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GLCoord4D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
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

impl Into<WorldCoord> for na::Point4<f32> {
    fn into(self) -> WorldCoord {
        WorldCoord{
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl GLCoord4D {
    pub fn to_world_coord(self, transformer: &Transformer) -> WorldCoord {
        transformer.unproject(self)
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
    pub fn to_gl_coord_4d(self, transformer: &Transformer) -> GLCoord4D {
        transformer.project(self)
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

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::transformer::IsometricRotation;

    #[test]   
    fn physical_position_to_gl_2d_left_top() {
        let physical_size = glutin::dpi::PhysicalSize::new(100.0, 50.0);
        let physical_position = glutin::dpi::PhysicalPosition::new(0.0, 0.0);
       
        assert_eq!(physical_position.to_gl_coord_2d(physical_size), GLCoord2D{x: -1.0, y: 1.0});
    }

    #[test]   
    fn physical_position_to_gl_2d_right_top() {
        let physical_size = glutin::dpi::PhysicalSize::new(100.0, 50.0);
        let physical_position = glutin::dpi::PhysicalPosition::new(100.0, 0.0);
       
        assert_eq!(physical_position.to_gl_coord_2d(physical_size), GLCoord2D{x: 1.0, y: 1.0});
    }

    #[test]   
    fn physical_position_to_gl_2d_left_bottom() {
        let physical_size = glutin::dpi::PhysicalSize::new(100.0, 50.0);
        let physical_position = glutin::dpi::PhysicalPosition::new(0.0, 50.0);
       
        assert_eq!(physical_position.to_gl_coord_2d(physical_size), GLCoord2D{x: -1.0, y: -1.0});
    }

    #[test]   
    fn physical_position_to_gl_2d_right_bottom() {
        let physical_size = glutin::dpi::PhysicalSize::new(100.0, 50.0);
        let physical_position = glutin::dpi::PhysicalPosition::new(100.0, 50.0);
       
        assert_eq!(physical_position.to_gl_coord_2d(physical_size), GLCoord2D{x: 1.0, y: -1.0});
    }

    #[test]   
    fn physical_position_to_gl_2d_center() {
        let physical_size = glutin::dpi::PhysicalSize::new(100.0, 50.0);
        let physical_position = glutin::dpi::PhysicalPosition::new(50.0, 25.0);
       
        assert_eq!(physical_position.to_gl_coord_2d(physical_size), GLCoord2D{x: 0.0, y: 0.0});
    }

    // #[test]
    // fn test_gl_2d_to_gl_4d() {

    //     let gl_coord_2d = GLCoord2D{x: 4.0, y: 3.0};

    //     struct MockZFinder {}
    //     impl ZFinder for MockZFinder {
    //         fn get_z_at(&self, _: GLCoord2D) -> f32 {
    //             2.0
    //         }
    //     }
    //     let z_finder = MockZFinder{};

    //     assert_eq!(gl_coord_2d.to_gl_coord_4d(&z_finder), GLCoord4D{x: 4.0, y: 3.0, z: 2.0, w: 1.0});
    // }

    #[test]
    fn test_na_point4_to_gl_4d() {
        let point_4 = na::Point4::new(1.0, 2.0, 3.0, 4.0);
        let gl_coord_4: GLCoord4D = point_4.into();
        assert_eq!(gl_coord_4, GLCoord4D{x: 1.0, y: 2.0, z: 3.0, w: 4.0});
    }

      #[test]
    fn test_na_point4_to_world() {
        let point_4 = na::Point4::new(1.0, 2.0, 3.0, 4.0);
        let world_coord: WorldCoord = point_4.into();
        assert_eq!(world_coord, WorldCoord{x: 1.0, y: 2.0, z: 3.0});
    }

     #[test]
    fn test_gl_4d_to_na_point4() {
        let gl_coord_4: GLCoord4D = GLCoord4D{x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        let point_4: na::Point4<f32> = gl_coord_4.into();
        assert_eq!(point_4, na::Point4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_gl_4d_to_world() {
        let mut transformer = Transformer{
            scale: GLCoord2D{x: 1.0, y: 2.0},
            translation: GLCoord2D{x: 3.0, y: 4.0},
            rotation: IsometricRotation::TopLeftAtTop,
            projection_matrix: na::Matrix4::identity(),
            inverse_matrix: na::Matrix4::identity(),
        };

        transformer.compute_transform_matrix(0.0);

        let gl_coord_4 = GLCoord4D{x: 5.0, y: 6.0, z: 7.0, w: 8.0};
        let expected = transformer.unproject(gl_coord_4);
       
        assert_eq!(gl_coord_4.to_world_coord(&transformer), expected);
    }

    #[test]
    fn test_world_to_na_point4() {
        let world_coord = WorldCoord{x: 1.0, y: 2.0, z: 3.0};
        let point_4: na::Point4<f32> = world_coord.into();
        assert_eq!(point_4, na::Point4::new(1.0, 2.0, 3.0, 1.0));
    }

    #[test]
    fn test_world_to_gl_4d() {
        let mut transformer = Transformer{
            scale: GLCoord2D{x: 1.0, y: 2.0},
            translation: GLCoord2D{x: 3.0, y: 4.0},
            rotation: IsometricRotation::TopLeftAtTop,
            projection_matrix: na::Matrix4::identity(),
            inverse_matrix: na::Matrix4::identity(),
        };

        transformer.compute_transform_matrix(0.0);

        let world_coord = WorldCoord{x: 5.0, y: 6.0, z: 7.0};
        let expected = transformer.project(world_coord);
       
        assert_eq!(world_coord.to_gl_coord_4d(&transformer), expected);
    }

}