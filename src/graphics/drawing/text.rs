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

        let h = 0.05;
        let w = 0.05;
        let mut xo = 0.0;
    
        let mut vertices = vec![];

        for character in text.chars() {
            let (top_left, bottom_right) = font.get_texture_coords(character);
            let p = position;
             
            vertices.append(&mut vec![
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo - w, -h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, top_left.y, xo - w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xo - w, -h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xo + w, h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, bottom_right.y, xo + w, -h,
            ]);
            xo += w * 2.0;
        }

        vbo.load(vertices);

        Text{vbo, font}
    }
}

 