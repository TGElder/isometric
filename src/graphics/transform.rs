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

pub struct Transform{
    scale: GLCoord3D,
    translation: GLCoord2D,
    rotation: IsometricRotation,
    projection_matrix: na::Matrix4<f32>,
    inverse_matrix: na::Matrix4<f32>,
}

impl Transform {
    
    pub fn new(scale: GLCoord3D, translation: GLCoord2D, rotation: IsometricRotation) -> Transform {
        Transform{
            scale,
            translation,
            rotation,
            projection_matrix: na::Matrix4::identity(),
            inverse_matrix: na::Matrix4::identity(),
        }
    }

    pub fn compute_projection_matrix(&mut self) {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::from_vec(vec![
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, self.scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,]
        ).transpose();

        let isometric_matrix = self.compute_isometric_matrix();
        self.projection_matrix = scale_matrix * isometric_matrix;
        self.inverse_matrix = self.projection_matrix.try_inverse().unwrap();
    }

    pub fn get_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix
    }

    fn compute_isometric_matrix(&self) -> na::Matrix4<f32> {
        let c = self.rotation.c();
        let s = self.rotation.s();
        na::Matrix4::from_vec(vec![
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 1.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,]
        ).transpose()
    }

    pub fn translate(&mut self, delta: GLCoord2D) {
        self.translation.x = self.translation.x + delta.x;
        self.translation.y = self.translation.y + delta.y;
    }

    fn transform_maintaining_center(&mut self, center: GLCoord4D, mut transformation: Box<FnMut(&mut Self) -> ()>) {
        let old_x = center.x;
        let old_y = center.y;
        let world_point = self.unproject(center);
        transformation(self);
        self.compute_projection_matrix();
        let center = self.project(world_point);
        self.translation.x += old_x - center.x;
        self.translation.y += old_y - center.y;
    }

    pub fn scale(&mut self, center: GLCoord4D, delta: GLCoord2D) {
        self.transform_maintaining_center(
            center,
            Box::new(move |transform| {
                transform.scale.x = transform.scale.x * delta.x;
                transform.scale.y = transform.scale.y * delta.y;
            })
        );
    }

    pub fn rotate(&mut self, center: GLCoord4D, direction: &'static Direction) {
        self.transform_maintaining_center(
            center,
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


#[cfg(test)]
mod tests {

    use super::IsometricRotation;
    use super::Direction;
    use super::Transform;
    use super::super::coords::*;

    #[test]   
    fn test_isometric_projection_with_top_left_at_top() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(0.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(1.0, -0.5, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 1.0, 0.0)), GLCoord4D::new(0.0, -1.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(0.0, 1.0, 0.0)), GLCoord4D::new(-1.0, -0.5, 0.0, 1.0));
    }

    #[test]   
    fn test_isometric_projection_with_top_right_at_top() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopRightAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(0.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(1.0, 0.5, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 1.0, 0.0)), GLCoord4D::new(2.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(0.0, 1.0, 0.0)), GLCoord4D::new(1.0, -0.5, 0.0, 1.0));
    }

    #[test]   
    fn test_isometric_projection_with_bottom_right_at_top() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::BottomRightAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(0.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(-1.0, 0.5, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 1.0, 0.0)), GLCoord4D::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(0.0, 1.0, 0.0)), GLCoord4D::new(1.0, 0.5, 0.0, 1.0));
    }

    #[test]   
    fn test_isometric_projection_with_bottom_left_at_top() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::BottomLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(0.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(-1.0, -0.5, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(1.0, 1.0, 0.0)), GLCoord4D::new(-2.0, 0.0, 0.0, 1.0));
        assert_eq!(transform.project(WorldCoord::new(0.0, 1.0, 0.0)), GLCoord4D::new(-1.0, 0.5, 0.0, 1.0));
    }

    #[test]   
    fn test_isometric_projection_with_z() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 10.0)), GLCoord4D::new(1.0, 9.5, -10.0, 1.0));
    }

    #[test]   
    fn test_x_translate() {
         let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(-1.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(0.0, -0.5, 0.0, 1.0));
    }

    #[test]   
    fn test_y_translate() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.5),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]   
    fn test_both_translate() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(-1.0, 0.5),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]   
    fn test_translate_method() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.translate(GLCoord2D::new(-1.0, 0.5));
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]   
    fn test_x_scale() {
        let mut transform = Transform::new(
            GLCoord3D::new(3.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(3.0, -0.5, 0.0, 1.0));
    }

     #[test]   
    fn test_y_scale() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 3.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(1.0, -1.5, 0.0, 1.0));
    }

    #[test]   
    fn test_z_scale() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 3.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 10.0)), GLCoord4D::new(1.0, 9.5, -30.0, 1.0));
    }

    #[test]   
    fn test_xy_scale() {
        let mut transform = Transform::new(
            GLCoord3D::new(3.0, 3.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(1.0, 0.0, 0.0)), GLCoord4D::new(3.0, -1.5, 0.0, 1.0));
    }


    #[test]   
    fn test_scale_method() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();
        transform.scale(GLCoord4D::new(1.0, -0.5, 0.0, 1.0), GLCoord2D::new(2.0, 3.0));
        transform.compute_projection_matrix();

        assert_eq!(transform.project(WorldCoord::new(0.0, 1.0, 0.0)), GLCoord4D::new(-3.0, -0.5, 0.0, 1.0));
    }

    #[test]   
    fn world_point_under_center_of_scaling_should_stay_the_same() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();
        let center_of_scaling = GLCoord4D::new(12.0, 34.0, 0.0, 1.0);
        let world_coord_at_center = transform.unproject(center_of_scaling);
        transform.scale(center_of_scaling, GLCoord2D::new(3.0, 3.0));
        transform.compute_projection_matrix();
        assert_eq!(transform.project(world_coord_at_center), center_of_scaling);
    }

    #[test]   
    fn test_rotation_clockwise() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, -2.0, 0.0, 1.0));

        let center_of_rotation = GLCoord4D::new(0.0, -1.0, 0.0, 1.0);

        transform.rotate(center_of_rotation, &Direction::Clockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(-2.0, -1.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::Clockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::Clockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(2.0, -1.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::Clockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, -2.0, 0.0, 1.0));
    }

    #[test]   
    fn test_rotation_anticlockwise() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, -2.0, 0.0, 1.0));

        let center_of_rotation = GLCoord4D::new(0.0, -1.0, 0.0, 1.0);

        transform.rotate(center_of_rotation, &Direction::AntiClockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(2.0, -1.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::AntiClockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, 0.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::AntiClockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(-2.0, -1.0, 0.0, 1.0));

        transform.rotate(center_of_rotation, &Direction::AntiClockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(WorldCoord::new(2.0, 2.0, 0.0)), GLCoord4D::new(0.0, -2.0, 0.0, 1.0));
    }

    #[test]   
    fn world_point_under_center_of_rotation_should_stay_the_same() {
        let mut transform = Transform::new(
            GLCoord3D::new(1.0, 1.0, 1.0),
            GLCoord2D::new(0.0, 0.0),
            IsometricRotation::TopLeftAtTop,
        );
        transform.compute_projection_matrix();
        let center_of_scaling = GLCoord4D::new(12.0, 34.0, 0.0, 1.0);
        let world_coord_at_center = transform.unproject(center_of_scaling);
        transform.rotate(center_of_scaling, &Direction::Clockwise);
        transform.compute_projection_matrix();
        assert_eq!(transform.project(world_coord_at_center), center_of_scaling);
    }

}