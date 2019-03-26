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
}

impl Canvas {
    pub fn new(texture: Texture) -> Canvas {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let vertices = vec![
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            0.0, 256.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0,
            256.0, 256.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            256.0, 256.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            256.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ];
    
        vbo.load(vertices);

        Canvas{texture, vbo}
    }
}
