use super::coords::*;
use std::sync::Arc;

pub trait Projection {
    fn compute_projection_matrix(&self) -> na::Matrix4<f32>;
}

#[derive(Clone, Copy)]
pub struct Isometric {
    pub yaw: f32,
    pub pitch: f32,
}

impl Isometric {
    pub fn new(yaw: f32, pitch: f32) -> Isometric {
        Isometric{yaw, pitch}
    }
}

impl Projection for Isometric {
    fn compute_projection_matrix(&self) -> na::Matrix4<f32> {
        let yc = self.yaw.cos();
        let ys = self.yaw.sin();
        let pc = self.pitch.cos();
        let ps = self.pitch.sin();
        na::Matrix4::from_vec(vec![
            yc, -ys, 0.0, 0.0,
            -ys * pc, -yc * pc, ps, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
            ]
        ).transpose()
    }
}

pub struct Identity {}

impl Projection for Identity {
    fn compute_projection_matrix(&self) -> na::Matrix4<f32> {
       na::Matrix4::identity()
    }
}

pub struct Transform {
    scale: GLCoord3D,
    translation: GLCoord2D,
    projection: Arc<Projection>,
}

impl Transform {
    pub fn new(scale: GLCoord3D, translation: GLCoord2D, projection: Arc<Projection>) -> Transform {
        Transform {
            scale,
            translation,
            projection,
        }
    }

    #[rustfmt::skip]
    pub fn compute_transformation_matrix(&self) -> na::Matrix4<f32> {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::from_vec(vec![
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, self.scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,]
        ).transpose();

        scale_matrix * self.projection.compute_projection_matrix()
    }

    pub fn compute_inverse_matrix(&self) -> na::Matrix4<f32> {
        self.compute_transformation_matrix().try_inverse().unwrap()
    }

    #[rustfmt::skip]
    pub fn get_scale_as_matrix(&self) -> na::Matrix3<f32> {
        na::Matrix3::new(
            self.scale.x, 0.0, 0.0,
            0.0, self.scale.y, 0.0,
            0.0, 0.0, self.scale.z,
        )
    }

    pub fn translate(&mut self, delta: GLCoord2D) {
        self.translation.x = self.translation.x + delta.x;
        self.translation.y = self.translation.y + delta.y;
    }

    fn transform_maintaining_center(
        &mut self,
        center: GLCoord4D,
        mut transformation: Box<FnMut(&mut Self) -> ()>,
    ) {
        let old_x = center.x;
        let old_y = center.y;
        let world_point = self.unproject(center);
        transformation(self);
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
            }),
        );
    }

    pub fn change_projection(&mut self, center: GLCoord4D, projection: Arc<Projection>) {
        self.transform_maintaining_center(
            center,
            Box::new(move |transform| {
                transform.projection = projection.clone();
            }),
        );
    }

    pub fn look_at(&mut self, world_coord: WorldCoord) {
        let gl_coord = world_coord.to_gl_coord_4d(self);
        self.translate(GLCoord2D::new(-gl_coord.x, -gl_coord.y));
    }

    pub fn project(&self, world_coord: WorldCoord) -> GLCoord4D {
        let point: na::Point4<f32> = world_coord.into();
        (self.compute_transformation_matrix() * point).into()
    }

    pub fn unproject(&self, projected_coord: GLCoord4D) -> WorldCoord {
        let projected_point: na::Point4<f32> = projected_coord.into();
        (self.compute_inverse_matrix() * projected_point).into()
    }
}

// #[cfg(test)]
// mod tests {

//     use super::super::coords::*;
//     use super::Direction;
//     use super::IsometricRotation;
//     use super::Transform;

//     #[test]
//     fn test_isometric_projection_with_top_left_at_top() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(1.0, -0.5, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 1.0, 0.0)),
//             GLCoord4D::new(0.0, -1.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 1.0, 0.0)),
//             GLCoord4D::new(-1.0, -0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_isometric_projection_with_top_right_at_top() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopRight,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(1.0, 0.5, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 1.0, 0.0)),
//             GLCoord4D::new(2.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 1.0, 0.0)),
//             GLCoord4D::new(1.0, -0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_isometric_projection_with_bottom_right_at_top() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::BottomRight,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(-1.0, 0.5, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 1.0, 0.0)),
//             GLCoord4D::new(0.0, 1.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 1.0, 0.0)),
//             GLCoord4D::new(1.0, 0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_isometric_projection_with_bottom_left_at_top() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::BottomLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(-1.0, -0.5, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 1.0, 0.0)),
//             GLCoord4D::new(-2.0, 0.0, 0.0, 1.0)
//         );
//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 1.0, 0.0)),
//             GLCoord4D::new(-1.0, 0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_isometric_projection_with_z() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 10.0)),
//             GLCoord4D::new(1.0, 9.5, -10.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_x_translate() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(-1.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, -0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_y_translate() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.5),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(1.0, 0.0, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_both_translate() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(-1.0, 0.5),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_translate_method() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.translate(GLCoord2D::new(-1.0, 0.5));
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_x_scale() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(3.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(3.0, -0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_y_scale() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 3.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(1.0, -1.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_z_scale() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 3.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 10.0)),
//             GLCoord4D::new(1.0, 9.5, -30.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_xy_scale() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(3.0, 3.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(1.0, 0.0, 0.0)),
//             GLCoord4D::new(3.0, -1.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_scale_method() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();
//         transform.scale(
//             GLCoord4D::new(1.0, -0.5, 0.0, 1.0),
//             GLCoord2D::new(2.0, 3.0),
//         );
//         transform.compute_projection_matrix();

//         assert_eq!(
//             transform.project(WorldCoord::new(0.0, 1.0, 0.0)),
//             GLCoord4D::new(-3.0, -0.5, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn world_point_under_center_of_scaling_should_stay_the_same() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();
//         let center_of_scaling = GLCoord4D::new(12.0, 34.0, 0.0, 1.0);
//         let world_coord_at_center = transform.unproject(center_of_scaling);
//         transform.scale(center_of_scaling, GLCoord2D::new(3.0, 3.0));
//         transform.compute_projection_matrix();
//         assert_eq!(transform.project(world_coord_at_center), center_of_scaling);
//     }

//     #[test]
//     fn test_rotation_clockwise() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)),
//             GLCoord4D::new(0.0, -2.0, 0.0, 1.0)
//         );

//         let center_of_rotation = GLCoord4D::new(0.0, -1.0, 0.0, 1.0);

//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(-2.0, -1.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(2.0, -1.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(0.0, -2.0, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn test_rotation_anticlockwise() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)),
//             GLCoord4D::new(0.0, -2.0, 0.0, 1.0)
//         );

//         let center_of_rotation = GLCoord4D::new(0.0, -1.0, 0.0, 1.0);

//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(2.0, -1.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(0.0, 0.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(-2.0, -1.0, 0.0, 1.0)
//         );

//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         transform.rotate(center_of_rotation, Direction::AntiClockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(
//             transform.project(WorldCoord::new(2.0, 2.0, 0.0)).round(),
//             GLCoord4D::new(0.0, -2.0, 0.0, 1.0)
//         );
//     }

//     #[test]
//     fn world_point_under_center_of_rotation_should_stay_the_same() {
//         let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         transform.compute_projection_matrix();
//         let center_of_scaling = GLCoord4D::new(12.0, 34.0, 0.0, 1.0);
//         let world_coord_at_center = transform.unproject(center_of_scaling);
//         transform.rotate(center_of_scaling, Direction::Clockwise);
//         transform.compute_projection_matrix();
//         assert_eq!(transform.project(world_coord_at_center), center_of_scaling);
//     }

//     #[test]
//     pub fn test_look_at() {
//          let mut transform = Transform::new(
//             GLCoord3D::new(1.0, 1.0, 1.0),
//             GLCoord2D::new(0.0, 0.0),
//             IsometricRotation::TopLeft,
//         );
//         let world_coord = WorldCoord::new(12.0, 34.0, 100.0);
//         let gl_coord_4 = transform.project(world_coord);
//         assert!(gl_coord_4.x != 0.0);
//         assert!(gl_coord_4.y != 0.0);
//         transform.look_at(world_coord);
//         let gl_coord_4 = transform.project(world_coord);
//         assert!(gl_coord_4.x == 0.0);
//         assert!(gl_coord_4.y == 0.0);
//     }

// }
