use super::Drawing;
use super::super::vertex_objects::{VBO, ColoredVertex};

pub struct SeaDrawing {
    terrain_triangles: VBO<ColoredVertex>,
}

impl Drawing for SeaDrawing {
    fn draw(&self) {
        self.terrain_triangles.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }
}

impl SeaDrawing {
    pub fn new(width: f32, height: f32, level: f32) -> SeaDrawing {
        let mut out = SeaDrawing{
            terrain_triangles: VBO::new(gl::TRIANGLES),
        };
        let triangle_vertices = vec![
            0.0, 0.0, level, 0.0, 0.0, 1.0,
            0.0, height, level, 0.0, 0.0, 1.0,
            width, height, level, 0.0, 0.0, 1.0,
            0.0, 0.0, level, 0.0, 0.0, 1.0,
            width, height, level, 0.0, 0.0, 1.0,
            width, 0.0, level, 0.0, 0.0, 1.0,
        ];
        
        out.terrain_triangles.load(triangle_vertices);
        out
    }
}