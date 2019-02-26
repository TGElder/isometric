use super::super::coords::*;
use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, ColoredVertex};

pub struct SelectedCellDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for SelectedCellDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.001
    }
}

impl SelectedCellDrawing {
    pub fn select_cell(heights: &na::DMatrix<f32>, world_coordinate: WorldCoord) -> Option<SelectedCellDrawing> {
        let width = heights.shape().0 as f32;
        let height = heights.shape().1 as f32;
        let x = world_coordinate.x;
        let y = world_coordinate.y;

        if x < 0.0 || x >= width - 1.0 || y < 0.0 || y >= height - 1.0 {
            return None
        }

        let x = x as usize;
        let y = y as usize;

        let a = (x as f32, y as f32, heights[(x, y)]);
        let b = (x as f32 + 1.0, y as f32, heights[(x + 1, y)]);
        let c = (x as f32 + 1.0, y as f32 + 1.0, heights[(x + 1, y + 1)]);
        let d = (x as f32, y as f32 + 1.0, heights[(x, y + 1)]);

        let mut vbo = VBO::new(gl::TRIANGLES);

        vbo.load(
            vec![
                a.0, a.1, a.2 + 0.001, 1.0, 0.0, 0.0,
                d.0, d.1, d.2 + 0.001, 1.0, 0.0, 0.0,
                c.0, c.1, c.2 + 0.001, 1.0, 0.0, 0.0,
                a.0, a.1, a.2 + 0.001, 1.0, 0.0, 0.0,
                c.0, c.1, c.2 + 0.001, 1.0, 0.0, 0.0,
                b.0, b.1, b.2 + 0.001, 1.0, 0.0, 0.0,
            ]
        );

        Some(SelectedCellDrawing{vbo})
    }
}
