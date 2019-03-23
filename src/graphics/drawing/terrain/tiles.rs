use super::super::super::engine::Drawing;
use super::super::super::vertex_objects::{VBO, ColoredVertex};
use super::super::utils::*;
use ::terrain::Terrain;
use ::v2;

pub struct TerrainDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for TerrainDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }
}

impl TerrainDrawing {
    pub fn new(terrain: &Terrain, coloring: Box<SquareColoring>) -> TerrainDrawing {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];

        for x in 0..((terrain.width() - 1) / 2) {
            for y in 0..((terrain.height() - 1) / 2) {
                let tile_index = v2(x, y);
                let grid_index = Terrain::get_index_for_tile(&tile_index);
                let border = terrain.get_border(grid_index);
                let color = coloring.get_colors(&[border[0], border[1], border[2], border[3]])[0];
                for triangle in terrain.get_triangles_for_tile(&tile_index) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
                }
            }
        }

        vbo.load(vertices);

        TerrainDrawing{vbo}
    }
}