use super::Drawing;
use super::super::vertex_objects::{VBO, TexturedVertex};
use super::super::texture::Texture;

pub struct Canvas {
    vbo: VBO<TexturedVertex>,
    texture: Texture,
}

impl Drawing for Canvas {
    fn draw(&self) {
        unsafe {
            self.texture.bind();
            self.vbo.draw();
            self.texture.unbind();
        }
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }

    fn text(&self) -> bool {
        true
    }
}

impl Canvas {
    pub fn new(texture: Texture) -> Canvas {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let x = 0.1;
        let w = 0.1;
        let vertices = vec![
            x, x, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            x, x + w, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
            x + w, x + w, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            x, x, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            x + w, x + w, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            x + w, x, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ];
    
        vbo.load(vertices);

        Canvas{texture, vbo}
    }
}
