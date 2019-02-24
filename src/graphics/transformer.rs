use super::coords::*;

#[derive(Debug)]
pub enum Direction {
    Clockwise,
    AntiClockwise
}

#[derive(Debug)]
pub enum IsometricRotation {
    TopLeftAtTop,
    TopRightAtTop,
    BottomLeftAtTop,
    BottomRightAtTop
}

impl IsometricRotation {
    fn c(&self) -> f32 {
        match *self {
            IsometricRotation::TopLeftAtTop => 1.0,
            IsometricRotation::TopRightAtTop => 1.0,
            IsometricRotation::BottomLeftAtTop => -1.0,
            IsometricRotation::BottomRightAtTop => -1.0,
        }
    }

    fn s(&self) -> f32 {
        match *self {
            IsometricRotation::TopLeftAtTop => 1.0,
            IsometricRotation::TopRightAtTop => -1.0,
            IsometricRotation::BottomLeftAtTop => 1.0,
            IsometricRotation::BottomRightAtTop => -1.0,
        }
    }

    fn rotate(&self, direction: &Direction) -> IsometricRotation {
        match direction {
            Direction::Clockwise => match *self {
                IsometricRotation::TopLeftAtTop => IsometricRotation::BottomLeftAtTop,
                IsometricRotation::TopRightAtTop => IsometricRotation::TopLeftAtTop,
                IsometricRotation::BottomLeftAtTop => IsometricRotation::BottomRightAtTop,
                IsometricRotation::BottomRightAtTop => IsometricRotation::TopRightAtTop,
            }
            Direction::AntiClockwise => match *self {
                IsometricRotation::TopLeftAtTop => IsometricRotation::TopRightAtTop,
                IsometricRotation::TopRightAtTop => IsometricRotation::BottomRightAtTop,
                IsometricRotation::BottomLeftAtTop => IsometricRotation::TopLeftAtTop,
                IsometricRotation::BottomRightAtTop => IsometricRotation::BottomLeftAtTop,
            }
        }
    }

}

pub struct Transformer{
    pub scale: GLCoord2D,
    pub translation: GLCoord2D,
    pub rotation: IsometricRotation,
    projection_matrix: na::Matrix4<f32>,
    inverse_matrix: na::Matrix4<f32>,
}

impl Transformer {
    
    pub fn new(scale: GLCoord2D, translation: GLCoord2D, rotation: IsometricRotation) -> Transformer {
        Transformer{
            scale,
            translation,
            rotation,
            projection_matrix: na::Matrix4::identity(),
            inverse_matrix: na::Matrix4::identity(),
        }
    }

    pub fn compute_projection_matrix(&mut self, z_adjustment: f32) {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::new(
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = self.compute_isometric_matrix(z_adjustment);
        self.projection_matrix = scale_matrix * isometric_matrix;
        self.inverse_matrix = self.projection_matrix.try_inverse().unwrap();
    }

    pub fn get_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix
    }

    fn compute_isometric_matrix(&self, z_adjustment: f32) -> na::Matrix4<f32> {
        let c = self.rotation.c();
        let s = self.rotation.s();
        na::Matrix4::new(
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 128.0, 0.0,
            0.0, 0.0, -1.0 + z_adjustment, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translate(&mut self, delta: GLCoord2D) {
        self.translation.x = self.translation.x + delta.x;
        self.translation.y = self.translation.y + delta.y;
    }

    fn transform_maintaining_centre(&mut self, centre: GLCoord4D, mut transformation: Box<FnMut(&mut Self) -> ()>) {
        let old_x = centre.x;
        let old_y = centre.y;
        let world_point = self.unproject(centre);
        transformation(self);
        self.compute_projection_matrix(0.0);
        let centre = self.project(world_point);
        self.translation.x += old_x - centre.x;
        self.translation.y += old_y - centre.y;
    }

    pub fn scale(&mut self, centre: GLCoord4D, delta: GLCoord2D) {
        self.transform_maintaining_centre(
            centre,
            Box::new(move |transform| {
                transform.scale.x = transform.scale.x * delta.x;
                transform.scale.y = transform.scale.y * delta.y;
            })
        );
    }

    pub fn rotate(&mut self, centre: GLCoord4D, direction: &'static Direction) {
        self.transform_maintaining_centre(
            centre,
            Box::new(move |transform| {
                transform.rotation = transform.rotation.rotate(direction);
            })
        );
    }

    pub fn project(&self, world_coord: WorldCoord) -> GLCoord4D {
        let point: na::Point4<f32> = world_coord.into();
        (self.projection_matrix * point).into()
    }

    pub fn unproject(&self, projected_coord: GLCoord4D) -> WorldCoord {
        let projected_point: na::Point4<f32> = projected_coord.into();
        (self.inverse_matrix * projected_point).into()
    }

}


// #[cfg(test)]
// mod tests {

//     use super::IsometricRotation;
//     use super::Direction;
//     use super::Transformer;
//     use super::super::coords::*;

//     #[test]   
//     fn test_isometric_projection_with_top_left_at_top() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(1.0, -0.5, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
//             na::Point4::new(0.0, -1.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
//             na::Point4::new(-1.0, -0.5, 0.0, 1.0),
//         );
//     }

// }

//      #[test]   
//     fn test_isometric_projection_with_top_right_at_top() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopRightAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(1.0, 0.5, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
//             na::Point4::new(2.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
//             na::Point4::new(1.0, -0.5, 0.0, 1.0),
//         );
//     }

//         #[test]   
//     fn test_isometric_projection_with_bottom_right_at_top() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::BottomRightAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(-1.0, 0.5, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
//             na::Point4::new(0.0, 1.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
//             na::Point4::new(1.0, 0.5, 0.0, 1.0),
//         );
//     }

//        #[test]   
//     fn test_isometric_projection_with_bottom_left_at_top() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::BottomLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(-1.0, -0.5, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
//             na::Point4::new(-2.0, 0.0, 0.0, 1.0),
//         );
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
//             na::Point4::new(-1.0, 0.5, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_isometric_projection_with_z() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 10.0, 1.0),
//             na::Point4::new(1.0, 9.5, -10.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_x_scale() {
//         let transformer = Transformer{
//             scale: na::Point2::new(3.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(3.0, -0.5, 0.0, 1.0),
//         );
//     }

//      #[test]   
//     fn test_y_scale() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 3.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(1.0, -1.5, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_both_scale() {
//         let transformer = Transformer{
//             scale: na::Point2::new(3.0, 3.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(3.0, -1.5, 0.0, 1.0),
//         );
//     }

//      #[test]   
//     fn centre_of_scaling_should_not_move() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         let centre_of_scaling = transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0);

//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0),
//             centre_of_scaling,
//         );
//     }

//     #[test]   
//     fn test_scale_method() { // TODO only tests uniform scaling
//         let mut transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         transformer.scale(na::Point4::new(1.0, 1.0, 0.0, 1.0), GLCoord2D{x: 3.0, y: 3.0});
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(1.0, -3.5, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_x_translate() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(-1.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, -0.5, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_y_translate() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.5),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(1.0, 0.0, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_both_translate() {
//         let transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(-1.0, 0.5),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn test_translate_method() {
//         let mut transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         transformer.translate(na::Point2::new(-1.0, 0.5));
       
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );
//     }

//     #[test]   
//     fn centre_of_rotation_should_not_move() {
//         let mut transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         let centre_of_rotation = transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0);

//         transformer.rotate(centre_of_rotation, Direction::Clockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0),
//             centre_of_rotation,
//         );
//     }

//     #[test]   
//     fn test_rotation_clockwise() {

//         let mut transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, -2.0, 0.0, 1.0),
//         );

//         let centre_of_rotation = na::Point4::new(0.0, -1.0, 0.0, 1.0);

//         transformer.rotate(centre_of_rotation, Direction::Clockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(-2.0, -1.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::Clockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::Clockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(2.0, -1.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::Clockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, -2.0, 0.0, 1.0),
//         );
//     }

//      #[test]   
//     fn test_rotation_anticlockwise() {

//         let mut transformer = Transformer{
//             scale: na::Point2::new(1.0, 1.0),
//             translation: na::Point2::new(0.0, 0.0),
//             rotation: IsometricRotation::TopLeftAtTop,
//         };

//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, -2.0, 0.0, 1.0),
//         );

//         let centre_of_rotation = na::Point4::new(0.0, -1.0, 0.0, 1.0);

//         transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(2.0, -1.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, 0.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(-2.0, -1.0, 0.0, 1.0),
//         );

//         transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
//         assert_eq!(
//             transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
//             na::Point4::new(0.0, -2.0, 0.0, 1.0),
//         );
//     }

// }