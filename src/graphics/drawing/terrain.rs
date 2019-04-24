use super::super::engine::DrawingType;
use super::super::vertex_objects::VBO;
use super::utils::*;
use super::Drawing;
use color::Color;
use coords::WorldCoord;
use terrain::{Edge, Node, Terrain};
use {v2, V2, M};
use std::sync::Arc;

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
    pub fn new(terrain: &Terrain, nodes: &Vec<Edge>, color: Color, z_mod: f32) -> EdgeDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);

        let mut vertices = vec![];

        for node in nodes {
            for triangle in terrain.get_triangles(Terrain::get_index_for_edge(&node)) {
                vertices.append(&mut get_uniform_colored_vertices_from_triangle(
                    &triangle, &color,
                ));
            }
        }

        vbo.load(vertices);

        EdgeDrawing { vbo, z_mod }
    }
}

#[derive(Clone)]
pub struct TerrainDrawing {
    width: usize,
    height: usize,
    slab_size: usize,
    stride: usize,
    vbo: Arc<VBO>,
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

    pub fn new(width: usize, height: usize, slab_size: usize) -> TerrainDrawing {
        let mut vbo = VBO::new(DrawingType::Plain);
        let stride = std::mem::size_of::<f32>() * 9 * 2 * slab_size * slab_size * 4;
        println!("Max slab size = {}", stride);
        let capacity = stride * (width / slab_size) * (height / slab_size);
        println!("Estimated capacity = {}", capacity);
        vbo.clear(capacity);
        TerrainDrawing{ width, height, slab_size, stride, vbo: Arc::new(vbo) }
    }

    pub fn get_index(&self, from: V2<usize>) -> usize {
        ((self.width / self.slab_size) * (from.y / self.slab_size)) + (from.x / self.slab_size)
    }


    pub fn draw_on(
        &mut self,
        terrain: &Terrain,
        color_matrix: &M<Color>,
        shading: &Box<SquareColoring>,
        from: V2<usize>,
        to: V2<usize>,
    ) {
        let mut vertices = vec![];

        for x in from.x..to.x {
            for y in from.y..to.y {
                let tile_index = v2(x, y);
                let grid_index = Terrain::get_index_for_tile(&tile_index);
                let border = terrain.get_border(grid_index);
                let shade = shading.get_colors(&[border[0], border[1], border[2], border[3]])[0];
                let color = color_matrix[(x, y)].mul(&shade);
                for triangle in terrain.get_triangles_for_tile(&tile_index) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(
                        &triangle, &color,
                    ));
                }
            }
        }

        println!("Slab size = {}", std::mem::size_of::<f32>() * vertices.len());

        let index = self.get_index(from);
        let offset = index * self.stride;
        println!("Loading at offset {}", offset);
        self.vbo.load_at_offset(offset as isize, vertices);
    }
}