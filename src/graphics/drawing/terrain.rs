use super::super::engine::{Drawing, Color};
use super::super::vertex_objects::{VBO, Vertex, ColoredVertex};
use utils::float_ordering;
use std::f32;

pub trait Coloring {
    fn get_color(&self, x: usize, y: usize, heights: &na::DMatrix<f32>) -> Color;
}

pub struct AltitudeColoring {
    max_height: f32,
}

impl AltitudeColoring {
    pub fn new(heights: & na::DMatrix<f32>) -> AltitudeColoring {
        let max_height = heights.iter().max_by(float_ordering).unwrap();
        AltitudeColoring{max_height: *max_height}
    }
}

impl Coloring for AltitudeColoring {
    fn get_color(&self, x: usize, y: usize, heights: &na::DMatrix<f32>) -> Color {
        let z = heights[(x, y)];
        let color = (z / (self.max_height * 2.0)) + 0.5;
        Color::new(color, color, color, 1.0)
    }
}

pub struct AngleColoring {
    light_direction: na::Vector3<f32>,
}

impl AngleColoring {
    pub fn new(light_direction: na::Vector3<f32>) -> AngleColoring {
        AngleColoring{light_direction}
    }
}

impl Coloring for AngleColoring {
    fn get_color(&self, x: usize, y: usize, heights: &na::DMatrix<f32>) -> Color {
        let a = na::Vector3::new(x as f32, y as f32, heights[(x, y)]);
        let b = na::Vector3::new((x + 1) as f32, y as f32, heights[(x + 1, y)]);
        let c = na::Vector3::new((x + 1) as f32, (y + 1) as f32, heights[(x + 1, y + 1)]);
        let d = na::Vector3::new(x as f32, (y + 1) as f32, heights[(x, y + 1)]);
        let u = a - c;
        let v = b - d;
        let normal = u.cross(&v);
        let angle: f32 = na::Matrix::angle(&normal, &self.light_direction);
        let color = angle / f32::consts::PI;
        Color::new(color, color, color, 1.0)
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
    pub fn from_heights(heights: &na::DMatrix<f32>, coloring: Box<Coloring>) -> TerrainDrawing {
        let mut out = TerrainDrawing{
            terrain_triangles: VBO::new(gl::TRIANGLES),
        };
        out.terrain_triangles.load(TerrainDrawing::get_vertices(heights, coloring));
        out
    }

    fn get_vertices(heights: &na::DMatrix<f32>, coloring: Box<Coloring>) -> Vec<f32> {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut triangle_vertices: Vec<f32> = Vec::with_capacity(width * height * 36);

        let with_z_and_color = |x: usize, y: usize, zx: usize, zy: usize| -> (f32, f32, f32, f32, f32, f32) {
            let color = coloring.get_color(x, y, heights);
            let z = heights[(zx, zy)];
            (zx as f32, zy as f32, z, color.r, color.g, color.b)
        };


        for y in 0..(height - 1) {
            for x in 0..(width - 1) {

                let a = with_z_and_color(x, y, x, y);
                let b = with_z_and_color(x, y, x + 1, y);
                let c = with_z_and_color(x, y, x + 1, y + 1);
                let d = with_z_and_color(x, y, x, y + 1);

                triangle_vertices.extend([
                    a.0, a.1, a.2, a.3, a.4, a.5,
                    d.0, d.1, d.2, d.3, d.4, d.5,
                    c.0, c.1, c.2, c.3, c.4, c.5,
                    a.0, a.1, a.2, a.3, a.4, a.5,
                    c.0, c.1, c.2, c.3, c.4, c.5,
                    b.0, b.1, b.2, b.3, b.4, b.5
                ].iter().cloned());
            }
        }
        triangle_vertices
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

    #[test]   
    fn test_terrain_drawing_get_vertices() {
        let heights = na::DMatrix::from_row_slice(3, 3, &[
            90.0, 80.0, 70.0,
            60.0, 50.0, 40.0,
            30.0, 20.0, 100.0
        ]).transpose();

        let coloring = Box::new(AltitudeColoring::new(&heights));
        let actual = TerrainDrawing::get_vertices(&heights, coloring);

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