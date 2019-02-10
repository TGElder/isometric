pub struct Transformer{
    viewport_size: na::Point2<u32>,
    scale: na::Point2<f32>,
    translation: na::Point2<f32>,
    rotation: usize,
}

impl Transformer {
    
    const ISOMETRIC_COEFFS: [(f32, f32); 4] = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];

    pub fn new(viewport_size: na::Point2<u32>) -> Transformer {
        let mut out = Transformer{
            viewport_size,
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: 0,
        };
        out.set_viewport_size(viewport_size);
        out
    }

    pub fn compute_transform_matrix(&self, z_adjustment: f32) -> na::Matrix4<f32> {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::new(
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = Transformer::compute_isometric_matrix(self.rotation, z_adjustment);

        scale_matrix * isometric_matrix
    }

    pub fn set_viewport_size(&mut self, viewport_size: na::Point2<u32>) {
        let scale = na::Point2::new(
            self.scale.x * ((self.viewport_size.x as f32) / (viewport_size.x as f32)),
            self.scale.y * ((self.viewport_size.y as f32) / (viewport_size.y as f32))
        );

        self.viewport_size = viewport_size;
        self.scale = scale;
        unsafe {
            gl::Viewport(0, 0, viewport_size.x as i32, viewport_size.y as i32);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }
    }

    pub fn scale(&mut self, delta: f32) {
        self.scale = self.scale * delta;
    }

    pub fn translate(&mut self, delta: na::Point2<f32>) {
        self.translation = na::Point2::new(self.translation.x - delta.x, self.translation.y + delta.y);
    }

    fn compute_isometric_matrix(angle: usize, z_adjustment: f32) -> na::Matrix4<f32> {
        let c = Transformer::ISOMETRIC_COEFFS[angle].0;
        let s = Transformer::ISOMETRIC_COEFFS[angle].1;
        na::Matrix4::new(
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 16.0, 0.0,
            0.0, 0.0, -1.0 + z_adjustment, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate(&mut self, centre_of_rotation: na::Point2<f32>, rotations: usize) {
        self.rotation = (self.rotation + rotations) % Transformer::ISOMETRIC_COEFFS.len();
    }
}


#[cfg(test)]
mod tests {
    
    #[test]   
    fn test_scale() {
        let point: na::Point4<f32> = na::Point4::new(1.0, 1.0, 1.0, 1.0);
        let actual = 0;
    }

}