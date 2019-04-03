use ::coords::*;
use super::Drawing;
use super::super::engine::DrawingType;
use ::color::Color;
use super::super::vertex_objects::{VBO, ColoredVertex};
use super::utils::*;
use ::terrain::Terrain;
use ::v2;

pub struct SelectedCellDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for SelectedCellDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        -0.0001
    }
 
    fn drawing_type(&self) -> DrawingType {
        DrawingType::Plain
    }
}

impl SelectedCellDrawing {
    pub fn select_cell(terrain: &Terrain, world_coordinate: WorldCoord) -> Option<SelectedCellDrawing> {

        let color = Color::new(1.0, 0.0, 0.0, 1.0);

        let width = (terrain.width() / 2) as f32;
        let height = (terrain.height() / 2) as f32;
        let x = world_coordinate.x;
        let y = world_coordinate.y;


        if x < 0.0 || x >= width - 1.0 || y < 0.0 || y >= height - 1.0 {
            return None
        }

        let x = x as usize;
        let y = y as usize;

        let mut vertices = vec![];

        for triangle in terrain.get_triangles_for_tile(&v2(x, y)) {
            vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
        }

        let mut vbo = VBO::new(gl::TRIANGLES);

        vbo.load(vertices);

        Some(SelectedCellDrawing{vbo})
    }
}
