use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;
use super::Drawing;
use font::Font;
use std::sync::Arc;
use V3;

pub struct Text {
    vbo: VBO,
    font: Arc<Font>,
}

impl Drawing for Text {
    fn draw(&self) {
        unsafe {
            self.font.texture().bind();
            self.vbo.draw();
            self.font.texture().unbind();
        }
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }
}

impl Text {
    #[rustfmt::skip]
    pub fn new(text: &str, position: V3<f32>, font: Arc<Font>) -> Text {
        let mut vbo = VBO::new(DrawingType::Text);

        let mut vertices = vec![];

        let total_width: f32 = font.get_width(text) as f32;
        let mut xo = -total_width / 2.0;

        for character in text.chars() {
            let (top_left, bottom_right) = font.get_texture_coords(character);
            let p = position;
            let (w, h) = font.get_dimensions(character);
            let (w, h) = (w as f32, h as f32);

            vertices.append(&mut vec![
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo, 0.0,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, top_left.y, xo, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo, 0.0,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, bottom_right.y, xo + w, 0.0,
            ]);
            xo += font.get_advance(character) as f32;
        }

        vbo.load(vertices);

        Text{vbo, font}
    }
}
