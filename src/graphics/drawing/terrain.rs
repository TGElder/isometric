use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, Vertex, ColoredVertex};
use super::utils::{SquareColoring, get_colored_vertices_from_square};
use std::f32;

#[derive(Debug, PartialEq)]
pub struct River {
    from: na::Vector2<usize>,
    to: na::Vector2<usize>,
    width: f32,
}

impl River {
    pub fn new(from: na::Vector2<usize>, to: na::Vector2<usize>, width: f32) -> River {
        if (to.x > from.x) || (to.y > from.y) {
            assert!(to.x > from.x || to.y > from.y);
            River{from, to, width}
        } else {
            River{from: to, to: from, width}
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Offsets {
    up_right: na::Vector2<f32>,
    down_right: na::Vector2<f32>,
    down_left: na::Vector2<f32>,
    up_left: na::Vector2<f32>,
}

impl Offsets {
    fn all_zero() -> Offsets {
        Offsets{
            up_right: na::Vector2::new(0.0, 0.0),
            down_right: na::Vector2::new(0.0, 0.0),
            down_left: na::Vector2::new(0.0, 0.0),
            up_left: na::Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct RiverCompass {
    up: f32,
    right: f32,
    down: f32,
    left: f32,
}

impl RiverCompass {
    fn all_zero() -> RiverCompass {
        RiverCompass{up: 0.0, right: 0.0, down: 0.0, left: 0.0}
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

                let points = [
                    na::Vector3::new(x as f32 + offsets[(x, y)].up_right.x, y as f32 + offsets[(x, y)].up_right.y, heights[(x, y)]),
                    na::Vector3::new((x + 1) as f32 + offsets[(x + 1, y)].up_left.x, y as f32 + offsets[(x + 1, y)].up_left.y, heights[(x + 1, y)]),
                    na::Vector3::new((x + 1) as f32 + offsets[(x + 1, y + 1)].down_left.x, (y + 1) as f32 + offsets[(x + 1, y + 1)].down_left.y, heights[(x + 1, y + 1)]),
                    na::Vector3::new(x as f32 + offsets[(x, y + 1)].down_right.x, (y + 1) as f32 + offsets[(x, y + 1)].down_right.y, heights[(x, y + 1)]),
                ];

                triangle_vertices.extend(get_colored_vertices_from_square(&points, &coloring));

            }
        }
        triangle_vertices
    }

    fn get_offsets(heights: &na::DMatrix<f32>, rivers: &Vec<River>) -> na::DMatrix<Offsets> {
        let width = heights.shape().0;
        let height = heights.shape().1;

        let mut compasses = na::DMatrix::from_element(width, height, RiverCompass::all_zero());
        let mut offsets = na::DMatrix::from_element(width, height, Offsets::all_zero());
        
        for river in rivers {
            if river.from.x == river.to.x {
                compasses[(river.from.x, river.from.y)].up = river.width;
                compasses[(river.to.x, river.to.y)].down = river.width;
            } else {
                compasses[(river.from.x, river.from.y)].right = river.width;
                compasses[(river.to.x, river.to.y)].left = river.width;
            }
        }

        let diagonal_mod = (2.0 as f32).sqrt();

        for x in 0..width {
            for y in 0..height {
                let compass = &compasses[(x, y)];
                let offsets = &mut offsets[(x, y)];
    
                if compass.up > 0.0 && compass.right > 0.0 {
                    let diagonal = (( (compass.up + compass.right) / 2.0) * diagonal_mod).min(0.5);
                    offsets.up_right = na::Vector2::new(diagonal, diagonal);
                } else if compass.up > 0.0 && compass.down > 0.0 {
                    offsets.up_right = na::Vector2::new((compass.up + compass.down) / 2.0, 0.0);
                } else if compass.left > 0.0 && compass.right > 0.0 {
                    offsets.up_right = na::Vector2::new(0.0, (compass.left + compass.right) / 2.0);
                };

                if compass.down > 0.0 && compass.right > 0.0 {
                    let diagonal = (( (compass.down + compass.right) / 2.0) * diagonal_mod).min(0.5);
                    offsets.down_right = na::Vector2::new(diagonal, -diagonal);
                } else if compass.up > 0.0 && compass.down > 0.0 {
                    offsets.down_right = na::Vector2::new((compass.up + compass.down) / 2.0, 0.0);
                } else if compass.left > 0.0 && compass.right > 0.0 {
                    offsets.down_right = na::Vector2::new(0.0, -(compass.left + compass.right) / 2.0);
                };

                if compass.down > 0.0 && compass.left > 0.0 {
                    let diagonal = (( (compass.down + compass.left) / 2.0) * diagonal_mod).min(0.5);
                    offsets.down_left = na::Vector2::new(-diagonal, -diagonal);
                } else if compass.up > 0.0 && compass.down > 0.0 {
                    offsets.down_left  = na::Vector2::new(-(compass.up + compass.down) / 2.0, 0.0);
                } else if compass.left > 0.0 && compass.right > 0.0 {
                    offsets.down_left = na::Vector2::new(0.0, -(compass.left + compass.right) / 2.0);
                };

                if compass.up > 0.0 && compass.left > 0.0 {
                    let diagonal = (( (compass.up + compass.left) / 2.0) * diagonal_mod).min(0.5);
                    offsets.up_left = na::Vector2::new(-diagonal, diagonal);
                } else if compass.up > 0.0 && compass.down > 0.0 {
                    offsets.up_left = na::Vector2::new(-(compass.up + compass.down) / 2.0, 0.0);
                } else if compass.left > 0.0 && compass.right > 0.0 {
                    offsets.up_left = na::Vector2::new(0.0, (compass.left + compass.right) / 2.0);
                };
            }
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


pub struct RiverDebugDrawing {
    terrain_lines: VBO<Vertex>,
}

impl Drawing for RiverDebugDrawing {
    fn draw(&self) {
        self.terrain_lines.draw();
    }

    fn get_z_mod(&self) -> f32 {
        -0.0002
    }
}

impl RiverDebugDrawing {
    pub fn new(heights: &na::DMatrix<f32>, rivers: &Vec<River>) -> TerrainGridDrawing {
        let mut out = TerrainGridDrawing{
            terrain_lines: VBO::new(gl::LINES),
        };

        let mut line_vertices: Vec<f32> = vec![];

        for river in rivers {
            line_vertices.append(&mut vec![
                river.from.x as f32, river.from.y as f32, heights[(river.from.x, river.from.y)],
                river.to.x as f32, river.to.y as f32, heights[(river.to.x, river.to.y)],
            ]);
            
        }

        out.terrain_lines.load(line_vertices);
        out
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