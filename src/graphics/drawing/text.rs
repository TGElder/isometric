use super::Drawing;
use super::super::vertex_objects::{VBO, TexturedVertex};
use ::font::Font;
use ::{V3};

pub struct Text {
    vbo: VBO<TexturedVertex>,
    font: Font,
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

    fn text(&self) -> bool {
        true
    }
}

impl Text {
    pub fn new(text: &str, position: V3<f32>, font: Font) -> Text {
        let mut vbo = VBO::new(gl::TRIANGLES);

    
        let mut vertices = vec![];

        let total_width: f32 = text.chars().map(|character| font.get_glyph(character).xadvance as f32).sum();
        let mut xo = -total_width / 2.0;


        for character in text.chars() {
            let (top_left, bottom_right) = font.get_texture_coords(character);
            let p = position;
            let glyph = font.get_glyph(character);
            let w = glyph.width as f32;
            let h = glyph.height as f32;
             
            vertices.append(&mut vec![
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo, 0.0,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, top_left.y, xo, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo, 0.0,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, bottom_right.y, xo + w, 0.0,
            ]);
            xo += glyph.xadvance as f32;
        }

        vbo.load(vertices);

        Text{vbo, font}
    }
}

 