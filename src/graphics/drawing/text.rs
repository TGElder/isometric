use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;
use super::Drawing;
use font::Font;
use std::sync::Arc;
use ::coords::WorldCoord;

pub struct Text {
    vbo: VBO,
    font: Arc<Font>,
    world_coord: WorldCoord,
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

    fn get_visibility_check_coord(&self) -> Option<&WorldCoord> {
        Some(&self.world_coord)
    }

}

impl Text {
    #[rustfmt::skip]
    pub fn new(text: &str, world_coord: WorldCoord, font: Arc<Font>) -> Text {
        let mut vbo = VBO::new(DrawingType::Text);

        let mut vertices = vec![];

        let total_width: f32 = font.get_width(text) as f32;
        let mut s = -total_width / 2.0;

        for character in text.chars() {
            let (top_left, bottom_right) = font.get_texture_coords(character);
            let p = world_coord;
            let (w, h) = font.get_dimensions(character);
            let (w, h) = (w as f32, h as f32);
            let (ox, oy) = font.get_offset(character);
            let xs = s + 0 as f32;
            let ys = 0 as f32;

            vertices.append(&mut vec![
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xs, ys,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, top_left.y, xs, ys + h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xs + w, ys + h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, top_left.x, bottom_right.y, xs, ys,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, top_left.y, xs + w, ys + h,
                p.x, p.y, p.z, 1.0, 1.0, 1.0, bottom_right.x, bottom_right.y, xs + w, ys,
            ]);

            s += font.get_advance(character) as f32;
        }

        vbo.load(vertices);

        Text{vbo, font, world_coord}
    }
}
