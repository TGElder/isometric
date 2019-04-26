use super::super::engine::DrawingType;
use super::super::vertex_objects::{MultiVBO, SimpleVBO};
use super::utils::*;
use super::Drawing;
use color::Color;
use coords::WorldCoord;
use terrain::{Edge, Node, Terrain};
use {v2, V2, M};

pub struct NodeDrawing {
    vbo: SimpleVBO,
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
        let mut vbo = SimpleVBO::new(DrawingType::Plain);

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
    vbo: SimpleVBO,
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
        let mut vbo = SimpleVBO::new(DrawingType::Plain);

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
    vbo: MultiVBO,
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
        
        let max_floats_per_index = 
            9 * // 9 floats per triangle
            2 * // 2 triangles per cell
            slab_size * slab_size * 4; // cells per slab
        println!("Max floats per index = {}", max_floats_per_index);
        let indices = (width * height) / (slab_size * slab_size);
        println!("Indices {}", indices);
        let vbo = MultiVBO::new(DrawingType::Plain, indices, max_floats_per_index);
        TerrainDrawing{ width, height, slab_size, vbo }
    }

    pub fn get_index(&self, from: V2<usize>) -> usize {
        ((self.width / self.slab_size) * (from.y / self.slab_size)) + (from.x / self.slab_size)
    }


    pub fn update(
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

        let index = self.get_index(from);
       
        self.vbo.load(index, vertices);
    }
}