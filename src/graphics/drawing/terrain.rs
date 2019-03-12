use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, Vertex, ColoredVertex};
use super::utils::{SquareColoring, get_colored_vertices_from_square};
use std::f32;

#[derive(Clone)]
struct Offsets {
    x_low: f32,
    x_high: f32,
    y_low: f32,
    y_high: f32,
}

impl Offsets {
    fn all_zero() -> Offsets {
        Offsets{x_low: 0.0, x_high: 0.0, y_low: 0.0, y_high: 0.0}
    }

    fn are_all_zero(&self) -> bool {
        self.x_low == 0.0 && self.x_high == 0.0 && self.y_low == 0.0 && self.y_high == 0.0
    }
}

pub struct TerrainDrawing {
    terrain_triangles: VBO<ColoredVertex>,
}

impl Drawing for TerrainDrawing {
    fn draw(&self) {
        self.terrain_triangles.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }
}

impl TerrainDrawing {
    pub fn new(heights: &na::DMatrix<f32>, rivers: &Vec<River>, coloring: Box<SquareColoring>) -> TerrainDrawing {
        let mut out = TerrainDrawing{
            terrain_triangles: VBO::new(gl::TRIANGLES),
        };
        out.terrain_triangles.load(TerrainDrawing::get_vertices(heights, rivers, coloring));
        out
    }

    fn get_vertices(heights: &na::DMatrix<f32>, rivers: &Vec<River>, coloring: Box<SquareColoring>) -> Vec<f32> {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut triangle_vertices: Vec<f32> = Vec::with_capacity(width * height * 36);

        let offsets = TerrainDrawing::get_offsets(heights, rivers);

        for y in 0..(height - 1) {
            for x in 0..(width - 1) {

                let offsets = &offsets[x][y];
                let x_low = x as f32 + offsets.x_low;
                let x_high = x as f32 + 1.0 - offsets.x_high;
                let y_low = y as f32 + offsets.y_low;
                let y_high = y as f32 + 1.0 - offsets.y_high;

                let points = [
                    na::Vector3::new(x_low, y_low, heights[(x, y)]),
                    na::Vector3::new(x_high, y_low, heights[(x + 1, y)]),
                    na::Vector3::new(x_high, y_high, heights[(x + 1, y + 1)]),
                    na::Vector3::new(x_low, y_high, heights[(x, y + 1)])
                ];

                triangle_vertices.extend(get_colored_vertices_from_square(&points, &coloring));

                if (!offsets.are_all_zero()) {

                }
            }
        }
        triangle_vertices
    }

    fn get_offsets(heights: &na::DMatrix<f32>, rivers: &Vec<River>) -> Vec<Vec<Offsets>> {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut offsets = vec![vec![Offsets::all_zero(); width]; height];

        let get_x_high = |river: &River| -> Option<(usize, usize)> {
            if river.from.x == river.to.x && river.from.x > 0 {
                Some((river.to.x - 1, river.to.y))
            } else {
                None
            }
        };

        let get_x_low = |river: &River| -> Option<(usize, usize)> {
            if river.from.x == river.to.x && river.from.x < width - 1 {
                Some((river.to.x, river.to.y))
            } else {
                None
            }
        };

        let get_y_high = |river: &River| -> Option<(usize, usize)> {
            if river.from.y == river.to.y && river.from.y > 0 {
                Some((river.to.x, river.to.y - 1))
            } else {
                None
            }
        };

        let get_y_low = |river: &River| -> Option<(usize, usize)> {
            if river.from.y == river.to.y && river.from.x < height - 1 {
                Some((river.to.x, river.to.y))
            } else {
                None
            }
        };

        let offset = 0.25;

        for river in rivers {
            get_x_high(&river).iter().for_each(|p| offsets[p.0][p.1].x_high = offset);
            get_x_low(&river).iter().for_each(|p| offsets[p.0][p.1].x_low = offset);
            get_y_high(&river).iter().for_each(|p| offsets[p.0][p.1].y_high = offset);
            get_y_low(&river).iter().for_each(|p| offsets[p.0][p.1].y_low = offset);
        }

        offsets
    }

    

}

pub struct TerrainGridDrawing {
    terrain_lines: VBO<Vertex>,
}

impl Drawing for TerrainGridDrawing {
    fn draw(&self) {
        self.terrain_lines.draw();
    }

    fn get_z_mod(&self) -> f32 {
        -0.0002
    }
}

impl TerrainGridDrawing {
    pub fn from_heights(heights: &na::DMatrix<f32>) -> TerrainGridDrawing {
        let mut out = TerrainGridDrawing{
            terrain_lines: VBO::new(gl::LINES),
        };
        out.terrain_lines.load(TerrainGridDrawing::get_vertices(heights));
        out
    }

    fn get_vertices(heights: &na::DMatrix<f32>) -> Vec<f32> {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut line_vertices: Vec<f32> = Vec::with_capacity(width * height * 24);
        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                
                let a = (x as f32, y as f32, heights[(x, y)]);
                let a = (a.0, a.1, a.2, (a.2 / 2.0) + 0.5);
                let b = (x as f32 + 1.0, y as f32, heights[(x + 1, y)]);
                let b = (b.0, b.1, b.2, (b.2 / 2.0) + 0.5);
                let c = (x as f32 + 1.0, y as f32 + 1.0, heights[(x + 1, y + 1)]);
                let c = (c.0, c.1, c.2, (c.2 / 2.0) + 0.5);
                let d = (x as f32, y as f32 + 1.0, heights[(x, y + 1)]);
                let d = (d.0, d.1, d.2, (d.2 / 2.0) + 0.5);

                line_vertices.extend([
                    a.0, a.1, a.2,
                    b.0, b.1, b.2,
                    b.0, b.1, b.2,
                    c.0, c.1, c.2,
                    c.0, c.1, c.2,
                    d.0, d.1, d.2,
                    d.0, d.1, d.2,
                    a.0, a.1, a.2   
                ].iter().cloned());
            }
        }
        line_vertices
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::utils::AltitudeSquareColoring;

    #[test]   
    fn test_terrain_drawing_get_vertices() {
        let heights = na::DMatrix::from_row_slice(3, 3, &[
            90.0, 80.0, 70.0,
            60.0, 50.0, 40.0,
            30.0, 20.0, 100.0
        ]).transpose();

        let coloring = Box::new(AltitudeSquareColoring::new(&heights));
        let actual = TerrainDrawing::get_vertices(&heights, &vec![], coloring);

        let expected = vec![
            0.0, 0.0, 90.0, 0.95, 0.95, 0.95,
            0.0, 1.0, 60.0, 0.8, 0.8, 0.8,
            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,
            0.0, 0.0, 90.0, 0.95, 0.95, 0.95,
            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,
            1.0, 0.0, 80.0, 0.9, 0.9, 0.9,

            1.0, 0.0, 80.0, 0.9, 0.9, 0.9,
            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,
            2.0, 1.0, 40.0, 0.7, 0.7, 0.7,
            1.0, 0.0, 80.0, 0.9, 0.9, 0.9,
            2.0, 1.0, 40.0, 0.7, 0.7, 0.7,
            2.0, 0.0, 70.0, 0.85, 0.85, 0.85,

            0.0, 1.0, 60.0, 0.8, 0.8, 0.8,
            0.0, 2.0, 30.0, 0.65, 0.65, 0.65,
            1.0, 2.0, 20.0, 0.6, 0.6, 0.6,
            0.0, 1.0, 60.0, 0.8, 0.8, 0.8,
            1.0, 2.0, 20.0, 0.6, 0.6, 0.6,
            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,

            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,
            1.0, 2.0, 20.0, 0.6, 0.6, 0.6,
            2.0, 2.0, 100.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 50.0, 0.75, 0.75, 0.75,
            2.0, 2.0, 100.0, 1.0, 1.0, 1.0,
            2.0, 1.0, 40.0, 0.7, 0.7, 0.7
        ];

        assert_eq!(actual, expected);
    }

    #[test]   
    fn test_terrain_drawing_grid_get_vertices() {
       let heights = na::DMatrix::from_row_slice(3, 3, &[
            0.9, 0.8, 0.7,
            0.6, 0.5, 0.4,
            0.3, 0.2, 0.1
        ]).transpose();

        let actual = TerrainGridDrawing::get_vertices(&heights);

        let expected = vec![
            0.0, 0.0, 0.9,
            1.0, 0.0, 0.8,
            1.0, 0.0, 0.8,
            1.0, 1.0, 0.5,
            1.0, 1.0, 0.5,
            0.0, 1.0, 0.6,
            0.0, 1.0, 0.6,
            0.0, 0.0, 0.9,

            1.0, 0.0, 0.8,
            2.0, 0.0, 0.7,
            2.0, 0.0, 0.7,
            2.0, 1.0, 0.4,
            2.0, 1.0, 0.4,
            1.0, 1.0, 0.5,
            1.0, 1.0, 0.5,
            1.0, 0.0, 0.8,

            0.0, 1.0, 0.6,
            1.0, 1.0, 0.5,
            1.0, 1.0, 0.5,
            1.0, 2.0, 0.2,
            1.0, 2.0, 0.2,
            0.0, 2.0, 0.3,
            0.0, 2.0, 0.3,
            0.0, 1.0, 0.6,

            1.0, 1.0, 0.5,
            2.0, 1.0, 0.4,
            2.0, 1.0, 0.4,
            2.0, 2.0, 0.1,
            2.0, 2.0, 0.1,
            1.0, 2.0, 0.2,
            1.0, 2.0, 0.2,
            1.0, 1.0, 0.5,
        ];

        assert_eq!(actual, expected);
    }
}