use super::Drawing;
use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;

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

        let triangle_vertices = vec![
            0.0, 0.0, level, 0.0, 0.0, 1.0,
            0.0, height, level, 0.0, 0.0, 1.0,
            width, height, level, 0.0, 0.0, 1.0,
            0.0, 0.0, level, 0.0, 0.0, 1.0,
            width, height, level, 0.0, 0.0, 1.0,
            width, 0.0, level, 0.0, 0.0, 1.0,
        ];
        
        vbo.load(triangle_vertices);
        SeaDrawing{vbo}
    }
}