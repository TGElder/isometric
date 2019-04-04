use super::Drawing;
use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;
use super::utils::*;
use ::color::Color;
use ::v3;

pub struct SeaDrawing {
    vbo: VBO,
}

impl Drawing for SeaDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }
}

impl SeaDrawing {
    pub fn new(width: f32, height: f32, level: f32) -> SeaDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);

        let color = Color::new(0.0, 0.0, 1.0, 1.0);

        vbo.load(get_uniform_colored_vertices_from_square(&[
                v3(0.0, 0.0, level),
                v3(width, 0.0, level),
                v3(width, height, level),
                v3(0.0, height, level),
            ], &color)
        );

        SeaDrawing{vbo}
    }
}