use super::super::super::engine::{Color, Drawing};
use super::super::super::vertex_objects::{VBO, ColoredVertex};
use super::super::utils::*;
use ::terrain::Terrain;
use ::{v2, V2, V3};

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
                let grid_index = Terrain::get_index_for_tile(tile_index);
                let border = terrain.get_border(grid_index);
                let color = coloring.get_colors(&[border[0], border[1], border[2], border[3]])[0];
                for triangle in trianglulate(terrain, v2(x, y), grid_index) {
                    vertices.append(&mut get_uniform_colored_vertices_from_triangle(&triangle, &color));
                }
            }
        }

        vbo.load(vertices);

        TerrainDrawing{vbo}
    }
}

fn trianglulate(terrain: &Terrain, tile_index: V2<usize>, grid_index: V2<usize>) -> Vec<[V3<f32>; 3]> {
    let mut out = vec![];
    out.append(&mut terrain.get_triangles(grid_index));

    let adjacents = vec![
        v2(grid_index.x - 1, grid_index.y),
        v2(grid_index.x + 1, grid_index.y),
        v2(grid_index.x, grid_index.y - 1),
        v2(grid_index.x, grid_index.y + 1),
    ];

    let min = v2(tile_index.x as f32, tile_index.y as f32);
    let max = v2(tile_index.x as f32 + 1.0, tile_index.y as f32 + 1.0);

    for adjacent in adjacents {
        let triangles = terrain.get_triangles(adjacent);
        if triangles.len() == 1 {
            let mut triangle = triangles[0];
            for p in 0..3 {
                triangle[p] = clip(triangle[p], &min, &max);
            }
            out.push(triangle);
        }
    }

    out
}

fn clip(mut point: V3<f32>, min: &V2<f32>, max: &V2<f32>) -> V3<f32> {
    point.x = point.x.max(min.x).min(max.x);
    point.y = point.y.max(min.y).min(max.y);

    point
}