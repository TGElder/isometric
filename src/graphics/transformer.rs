use super::coords::*;

pub enum Direction {
    Clockwise,
    AntiClockwise
}

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

    fn rotate(&self, direction: Direction) -> IsometricRotation {
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
    pub scale: na::Point2<f32>,
    pub translation: na::Point2<f32>,
    pub rotation: IsometricRotation,
}

impl Transformer {
    
    pub fn new(scale: na::Point2<f32>) -> Transformer {
        Transformer{
            scale,
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        }
    }

    pub fn compute_transform_matrix(&self, z_adjustment: f32) -> na::Matrix4<f32> {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::new(
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = self.compute_isometric_matrix(z_adjustment);

        scale_matrix * isometric_matrix
    }

    pub fn scale(&mut self, centre: na::Point4<f32>, delta: GLCoord2D) {
        let world_point = self.unproject(centre);
        self.scale.x = self.scale.x * delta.x;
        self.scale.y = self.scale.y * delta.y;
        let centre_new = self.compute_transform_matrix(0.0) * world_point;
        self.translation = na::Point2::new(
            (centre.x - centre_new.x) + self.translation.x,
            (centre.y - centre_new.y) + self.translation.y,
        );
    }

    pub fn translate(&mut self, delta: na::Point2<f32>) {
        self.translation = na::Point2::new(self.translation.x + delta.x, self.translation.y + delta.y);
    }

    pub fn compute_isometric_matrix(&self, z_adjustment: f32) -> na::Matrix4<f32> {
        let c = self.rotation.c();
        let s = self.rotation.s();
        na::Matrix4::new(
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 16.0, 0.0,
            0.0, 0.0, -1.0 + z_adjustment, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate(&mut self, centre: na::Point4<f32>, direction: Direction) {
        let world_point = self.unproject(centre);
        self.rotation = self.rotation.rotate(direction);
        let centre_new = self.compute_transform_matrix(0.0) * world_point;
        self.translation = na::Point2::new(
            (centre.x - centre_new.x) + self.translation.x,
            (centre.y - centre_new.y) + self.translation.y,
        );
    }

    pub fn project(&self, world_coord: WorldCoord) -> GLCoord4D {
        let point: na::Point4<f32> = world_coord.into();
        (self.compute_transform_matrix(0.0) * point).into()
    }

    pub fn unproject(&self, projected_point: na::Point4<f32>) -> na::Point4<f32> {
        let inverse = self.compute_transform_matrix(0.0).try_inverse().unwrap();
        inverse * projected_point
    }

}


#[cfg(test)]
mod tests {

    use super::IsometricRotation;
    use super::Direction;
    use super::Transformer;
    use super::super::coords::*;

    #[test]   
    fn test_isometric_projection_with_top_left_at_top() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(1.0, -0.5, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
            na::Point4::new(0.0, -1.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
            na::Point4::new(-1.0, -0.5, 0.0, 1.0),
        );
    }

     #[test]   
    fn test_isometric_projection_with_top_right_at_top() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopRightAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(1.0, 0.5, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
            na::Point4::new(2.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
            na::Point4::new(1.0, -0.5, 0.0, 1.0),
        );
    }

        #[test]   
    fn test_isometric_projection_with_bottom_right_at_top() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::BottomRightAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(-1.0, 0.5, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
            na::Point4::new(0.0, 1.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
            na::Point4::new(1.0, 0.5, 0.0, 1.0),
        );
    }

       #[test]   
    fn test_isometric_projection_with_bottom_left_at_top() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::BottomLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(-1.0, -0.5, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 1.0, 0.0, 1.0),
            na::Point4::new(-2.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(0.0, 1.0, 0.0, 1.0),
            na::Point4::new(-1.0, 0.5, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_isometric_projection_with_z() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 10.0, 1.0),
            na::Point4::new(1.0, 9.5, -10.0, 1.0),
        );
    }

    #[test]   
    fn test_x_scale() {
        let transformer = Transformer{
            scale: na::Point2::new(3.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(3.0, -0.5, 0.0, 1.0),
        );
    }

     #[test]   
    fn test_y_scale() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 3.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(1.0, -1.5, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_both_scale() {
        let transformer = Transformer{
            scale: na::Point2::new(3.0, 3.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(3.0, -1.5, 0.0, 1.0),
        );
    }

     #[test]   
    fn centre_of_scaling_should_not_move() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        let centre_of_scaling = transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0);

        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0),
            centre_of_scaling,
        );
    }

    #[test]   
    fn test_scale_method() { // TODO only tests uniform scaling
        let mut transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        transformer.scale(na::Point4::new(1.0, 1.0, 0.0, 1.0), GLCoord2D{x: 3.0, y: 3.0});
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(1.0, -3.5, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_x_translate() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(-1.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, -0.5, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_y_translate() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.5),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(1.0, 0.0, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_both_translate() {
        let transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(-1.0, 0.5),
            rotation: IsometricRotation::TopLeftAtTop,
        };
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
    }

    #[test]   
    fn test_translate_method() {
        let mut transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        transformer.translate(na::Point2::new(-1.0, 0.5));
       
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(1.0, 0.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );
    }

    #[test]   
    fn centre_of_rotation_should_not_move() {
        let mut transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        let centre_of_rotation = transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0);

        transformer.rotate(centre_of_rotation, Direction::Clockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(12.0, 34.0, 0.0, 1.0),
            centre_of_rotation,
        );
    }

    #[test]   
    fn test_rotation_clockwise() {

        let mut transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, -2.0, 0.0, 1.0),
        );

        let centre_of_rotation = na::Point4::new(0.0, -1.0, 0.0, 1.0);

        transformer.rotate(centre_of_rotation, Direction::Clockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(-2.0, -1.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::Clockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::Clockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(2.0, -1.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::Clockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, -2.0, 0.0, 1.0),
        );
    }

     #[test]   
    fn test_rotation_anticlockwise() {

        let mut transformer = Transformer{
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: IsometricRotation::TopLeftAtTop,
        };

        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, -2.0, 0.0, 1.0),
        );

        let centre_of_rotation = na::Point4::new(0.0, -1.0, 0.0, 1.0);

        transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(2.0, -1.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, 0.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(-2.0, -1.0, 0.0, 1.0),
        );

        transformer.rotate(centre_of_rotation, Direction::AntiClockwise);
        assert_eq!(
            transformer.compute_transform_matrix(0.0) * na::Point4::new(2.0, 2.0, 0.0, 1.0),
            na::Point4::new(0.0, -2.0, 0.0, 1.0),
        );
    }

}