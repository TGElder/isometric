use super::Drawing;
use ::color::Color;
use super::super::vertex_objects::{VBO, ColoredVertex};
use super::utils::*;
use ::terrain::{Node, Edge, Terrain};
use ::{v2, M};

pub struct NodeDrawing {
    vbo: VBO<ColoredVertex>,
    z_mod: f32,
}

impl Drawing for NodeDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        self.z_mod
    }

    fn text(&self) -> bool {
        false
    }
}

impl NodeDrawing {
    pub fn new(terrain: &Terrain, nodes: &Vec<Node>, color: Color, z_mod: f32) -> NodeDrawing {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];

        for node in nodes {
            for triangle in terrain.get_triangles(Terrain::get_index_for_node(&node)) {
                vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
            }
        }

        vbo.load(vertices);

        NodeDrawing{vbo, z_mod}
    }
}

pub struct EdgeDrawing {
    vbo: VBO<ColoredVertex>,
    z_mod: f32,
}

impl Drawing for EdgeDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        self.z_mod
    }

    fn text(&self) -> bool {
        false
    }
}

impl EdgeDrawing {
    pub fn new(terrain: &Terrain, nodes: &Vec<Edge>, color: Color, z_mod: f32) -> EdgeDrawing {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];

        for node in nodes {
            for triangle in terrain.get_triangles(Terrain::get_index_for_edge(&node)) {
                vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
            }
        }

        vbo.load(vertices);

        EdgeDrawing{vbo, z_mod}
    }
}

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

    fn text(&self) -> bool {
        false
    }
}

impl TerrainDrawing {
    pub fn from_matrix(terrain: &Terrain, color_matrix: &M<Color>, shading: &Box<SquareColoring>) -> TerrainDrawing {

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];

        for x in 0..((terrain.width() - 1) / 2) {
            for y in 0..((terrain.height() - 1) / 2) {
                let tile_index = v2(x, y);
                let grid_index = Terrain::get_index_for_tile(&tile_index);
                let border = terrain.get_border(grid_index);
                let shade = shading.get_colors(&[border[0], border[1], border[2], border[3]])[0];
                let color = color_matrix[(x, y)].mul(&shade);
                for triangle in terrain.get_triangles_for_tile(&tile_index) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
                }
            }
        }

        vbo.load(vertices);

        TerrainDrawing{vbo}
    }

    pub fn uniform(terrain: &Terrain, coloring: Box<SquareColoring>) -> TerrainDrawing {

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