use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;
use super::utils::*;
use super::Drawing;
use color::Color;
use terrain::{Edge, Node, Terrain};
use {v2, M};
use ::coords::WorldCoord;

pub struct NodeDrawing {
    vbo: VBO,
    z_mod: f32,
}

impl Drawing for NodeDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        self.z_mod
    }

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }
    
    fn get_visibility_check_coord(&self) -> Option<&WorldCoord> {
        None
    }
}

impl NodeDrawing {
    pub fn new(terrain: &Terrain, nodes: &Vec<Node>, color: Color, z_mod: f32) -> NodeDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);

        let mut vertices = vec![];

        for node in nodes {
            for triangle in terrain.get_triangles(Terrain::get_index_for_node(&node)) {
                vertices.append(&mut get_uniform_colored_vertices_from_triangle(
                    &triangle, &color,
                ));
            }
        }

        vbo.load(vertices);

        NodeDrawing { vbo, z_mod }
    }
}

pub struct EdgeDrawing {
    vbo: VBO,
    z_mod: f32,
}

impl Drawing for EdgeDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        self.z_mod
    }

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }

    fn get_visibility_check_coord(&self) -> Option<&WorldCoord> {
        None
    }
}

impl EdgeDrawing {
    pub fn new(terrain: &Terrain, edges: &Vec<Edge>, color: Color, z_mod: f32) -> EdgeDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);

        let mut vertices = vec![];

        for edge in edges {
            for triangle in terrain.get_triangles(Terrain::get_index_for_edge(&edge)) {
                vertices.append(&mut get_uniform_colored_vertices_from_triangle(
                    &triangle, &color,
                ));
            }
        }

        vbo.load(vertices);

        EdgeDrawing { vbo, z_mod }
    }
}

pub struct TerrainDrawing {
    vbo: VBO,
}

impl Drawing for TerrainDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }

    fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }
    
    fn get_visibility_check_coord(&self) -> Option<&WorldCoord> {
        None
    }
}

impl TerrainDrawing {

    pub fn uniform(terrain: &Terrain, coloring: Box<SquareColoring>) -> TerrainDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);

        let green = Color::new(0.0, 0.75, 0.0, 1.0);
        let grey = Color::new(0.5, 0.4, 0.3, 1.0);

        let mut vertices = vec![];

        for x in 0..terrain.width() - 1 {
            for y in 0..terrain.height() - 1 {
                let tile_index = v2(x, y);
                let border = terrain.get_border(tile_index);
                let base = if (border[0].z - border[1].z).abs() > 0.533333333 
                || (border[1].z - border[2].z).abs() > 0.533333333 
                || (border[2].z - border[3].z).abs() > 0.533333333 
                || (border[3].z - border[0].z).abs() > 0.533333333 {
                    grey
                } else {
                    green
                };
                let mut shade = coloring.get_colors(&[border[0], border[1], border[2], border[3]])[0];
                shade = shade.mul(&base);
                for triangle in terrain.get_triangles(tile_index) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(
                        &triangle, &shade,
                    ));
                }
            }
        }

        vbo.load(vertices);

        TerrainDrawing { vbo }
    }
}