use super::super::super::coords::*;
use super::super::super::engine::{Color, Drawing};
use super::super::super::vertex_objects::{VBO, ColoredVertex};
use super::super::utils::*;
use ::terrain::Terrain;
use ::v2;

pub struct NodeDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for NodeDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }
}

impl NodeDrawing {
    pub fn new(terrain: &Terrain, base_color: Color) -> NodeDrawing {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];

        for x in 0..((terrain.width() + 1) / 2) {
            for y in 0..((terrain.height() + 1) / 2) {
                for triangle in terrain.get_triangles(terrain.get_index_for_node(v2(x, y))) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &base_color));
                }
            }
        }

        vbo.load(vertices);

        NodeDrawing{vbo}
    }
}
