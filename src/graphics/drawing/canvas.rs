use super::Drawing;
use super::super::engine::DrawingType;
use super::super::texture::Texture;
use super::super::vertex_objects::VBO;

pub struct Canvas {
    vbo: VBO,
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

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }
}

impl Canvas {
    pub fn new(texture: Texture) -> Canvas {
        let mut vbo = VBO::new(DrawingType::Plain);

        let x = 344.0;
        let h = 0.025;
        let w = 0.05;
        let vertices = vec![
            x, x, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, -w, -h,
            x, x, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, -w, h,
            x, x, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, w, h,
            x, x, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, -w, -h,
            x, x, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, w, h,
            x, x, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, w, -h,
        ];

        // x, y, z, offset_x, offset_y, 

        vbo.load(vertices);

        Canvas{texture, vbo}
    }
}
