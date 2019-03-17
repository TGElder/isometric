use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, Vertex, ColoredVertex};
use super::utils::{SquareColoring, get_colored_vertices_from_square};
use std::f32;

#[derive(Debug, PartialEq)]
pub struct River {
    from: na::Vector2<usize>,
    to: na::Vector2<usize>,
    from_width: f32,
    to_width: f32,
}

impl River {
    pub fn new(from: na::Vector2<usize>, to: na::Vector2<usize>, from_width: f32, to_width: f32) -> River {
        River{from, to, from_width, to_width}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Offsets {
    right_up: na::Vector2<f32>,
    right_down: na::Vector2<f32>,
    left_down: na::Vector2<f32>,
    left_up: na::Vector2<f32>,
}

impl Offsets {
    fn all_zero() -> Offsets {
        Offsets{
            right_up: na::Vector2::new(0.0, 0.0),
            right_down: na::Vector2::new(0.0, 0.0),
            left_down: na::Vector2::new(0.0, 0.0),
            left_up: na::Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct RiverCompass {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
    width: f32,
}

impl RiverCompass {
    fn init() -> RiverCompass {
        RiverCompass{up: false, right: false, down: false, left: false, width: 0.0}
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

fn get_river_compasses(width: usize, height: usize, rivers: &Vec<River>) -> na::DMatrix<RiverCompass> {
    let mut compasses = na::DMatrix::from_element(width, height, RiverCompass::init());

    for river in rivers {
        compasses[(river.from.x, river.from.y)].width = river.from_width;
        compasses[(river.to.x, river.to.y)].width = river.to_width;
        if river.from.x == river.to.x {
            if river.from.y < river.to.y {
                compasses[(river.from.x, river.from.y)].up = true;
                compasses[(river.to.x, river.to.y)].down = true;
            } else {
                compasses[(river.from.x, river.from.y)].down = true;
                compasses[(river.to.x, river.to.y)].up = true;
            }
        } else {
            if river.from.x < river.to.x {
                compasses[(river.from.x, river.from.y)].right = true;
                compasses[(river.to.x, river.to.y)].left = true;
            } else {
                compasses[(river.from.x, river.from.y)].left = true;
                compasses[(river.to.x, river.to.y)].right = true;
            }
        }
    }

    compasses
}

fn get_offsets(width: usize, height: usize, rivers: &Vec<River>, square_t_junctions: bool) -> na::DMatrix<Offsets> {
    let compasses = get_river_compasses(width, height, rivers);
    let mut offsets = na::DMatrix::from_element(width, height, Offsets::all_zero());
    
    let diagonal_mod = (2.0 as f32).sqrt();

    for x in 0..width {
        for y in 0..height {
            let compass = &compasses[(x, y)];
            
            let half_width = compass.width / 2.0;

            let offsets = &mut offsets[(x, y)];

            let count  = 
                if compass.up {1} else {0} +
                if compass.right {1} else {0} +
                if compass.down {1} else {0} +
                if compass.left {1} else {0};

            if count == 1 {
                if compass.down {
                    offsets.left_down = na::Vector2::new(-half_width, 0.0);
                    offsets.right_down = na::Vector2::new(half_width, 0.0);
                } else if compass.up {
                    offsets.left_up = na::Vector2::new(-half_width, 0.0);
                    offsets.right_up = na::Vector2::new(half_width, 0.0);
                } else if compass.left {
                    offsets.left_down = na::Vector2::new(0.0, -half_width);
                    offsets.left_up = na::Vector2::new(0.0, half_width);
                } else if compass.right {
                    offsets.right_down = na::Vector2::new(0.0, -half_width);
                    offsets.right_up = na::Vector2::new(0.0, half_width);
                }
            } else {
                let diagonal = if count == 2 || !square_t_junctions {
                    (half_width * diagonal_mod).min(0.5)
                } else {
                    half_width
                };

                if compass.up && compass.right {
                    offsets.right_up = na::Vector2::new(diagonal, diagonal);
                } else if compass.up && compass.down {
                    offsets.right_up = na::Vector2::new(half_width, 0.0);
                } else if compass.left && compass.right {
                    offsets.right_up = na::Vector2::new(0.0, half_width);
                }

                if compass.down && compass.right {
                    offsets.right_down = na::Vector2::new(diagonal, -diagonal);
                } else if compass.up && compass.down {
                    offsets.right_down = na::Vector2::new(half_width, 0.0);
                } else if compass.left && compass.right {
                    offsets.right_down = na::Vector2::new(0.0, -half_width);
                };

                if compass.down && compass.left {
                    offsets.left_down = na::Vector2::new(-diagonal, -diagonal);
                } else if compass.up && compass.down {
                    offsets.left_down  = na::Vector2::new(-half_width, 0.0);
                } else if compass.left && compass.right {
                    offsets.left_down = na::Vector2::new(0.0, -half_width);
                };

                if compass.up && compass.left {
                    offsets.left_up = na::Vector2::new(-diagonal, diagonal);
                } else if compass.up && compass.down {
                    offsets.left_up = na::Vector2::new(-half_width, 0.0);
                } else if compass.left && compass.right {
                    offsets.left_up = na::Vector2::new(0.0, half_width);
                };
            }

           
        }
    }

    offsets
}

fn get_points(x: usize, y: usize, heights: &na::DMatrix<f32>, offsets: &na::DMatrix<Offsets>) -> [na::Vector3<f32>; 4] {
    [
        na::Vector3::new(x as f32 + offsets[(x, y)].right_up.x, y as f32 + offsets[(x, y)].right_up.y, heights[(x, y)]),
        na::Vector3::new((x + 1) as f32 + offsets[(x + 1, y)].left_up.x, y as f32 + offsets[(x + 1, y)].left_up.y, heights[(x + 1, y)]),
        na::Vector3::new((x + 1) as f32 + offsets[(x + 1, y + 1)].left_down.x, (y + 1) as f32 + offsets[(x + 1, y + 1)].left_down.y, heights[(x + 1, y + 1)]),
        na::Vector3::new(x as f32 + offsets[(x, y + 1)].right_down.x, (y + 1) as f32 + offsets[(x, y + 1)].right_down.y, heights[(x, y + 1)]),
    ]
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

        let offsets = get_offsets(width, height, rivers, true);

        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                let points = get_points(x, y, &heights, &offsets);
                triangle_vertices.extend(get_colored_vertices_from_square(&points, &coloring));
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
    fn test_get_river_compasses() {
        let rivers = vec![
            River::new(na::Vector2::new(1, 0), na::Vector2::new(1, 1), 1.0, 3.0),
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 2.0, 3.0),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 2), 3.0, 4.0),
            River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 2.0, 3.0),
        ];
        let compasses = get_river_compasses(3, 3, &rivers);
        assert_eq!(compasses[(0, 0)], RiverCompass{up: false, down: false, right: false, left: false, width: 0.0});
        assert_eq!(compasses[(1, 0)], RiverCompass{up: true, down: false, right: false, left: false, width: 1.0});
        assert_eq!(compasses[(0, 1)], RiverCompass{up: false, down: false, right: true, left: false, width: 2.0});
        assert_eq!(compasses[(1, 1)], RiverCompass{up: true, down: true, right: true, left: true, width: 3.0});
        assert_eq!(compasses[(1, 2)], RiverCompass{up: false, down: true, right: false, left: false, width: 4.0});
        assert_eq!(compasses[(2, 1)], RiverCompass{up: false, down: false, right: false, left: true, width: 2.0});
    }

    #[test]
    fn test_get_offsets_vertical_stub() {
        let rivers = vec![
            River::new(na::Vector2::new(1, 2), na::Vector2::new(1, 1), 0.1, 0.2),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(-0.1, 0.0),
            right_up: na::Vector2::new(0.1, 0.0),
            left_down: na::Vector2::new(0.0, 0.0),
            right_down: na::Vector2::new(0.0, 0.0),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_horizontal_stub() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(0.0, 0.1),
            right_up: na::Vector2::new(0.0, 0.0),
            left_down: na::Vector2::new(0.0, -0.1),
            right_down: na::Vector2::new(0.0, 0.0),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_vertical_continuing() {
        let rivers = vec![
            River::new(na::Vector2::new(1, 0), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 2), 0.2, 3.0),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(-0.1, 0.0),
            right_up: na::Vector2::new(0.1, 0.0),
            left_down: na::Vector2::new(-0.1, 0.0),
            right_down: na::Vector2::new(0.1, 0.0),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_horizontal_continuing() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(2, 1), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(0.0, 0.1),
            right_up: na::Vector2::new(0.0, 0.1),
            left_down: na::Vector2::new(0.0, -0.1),
            right_down: na::Vector2::new(0.0, -0.1),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_corner() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 0), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let diagonal = (2.0 as f32).sqrt() * 0.1;

        let expected = Offsets{
            left_up: na::Vector2::new(0.0, 0.0),
            right_up: na::Vector2::new(0.0, 0.0),
            left_down: na::Vector2::new(-diagonal, -diagonal),
            right_down: na::Vector2::new(0.0, 0.0),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_t_junction() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 0), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let diagonal = (2.0 as f32).sqrt() * 0.1;

        let expected = Offsets{
            left_up: na::Vector2::new(0.0, 0.1),
            right_up: na::Vector2::new(0.0, 0.1),
            left_down: na::Vector2::new(-diagonal, -diagonal),
            right_down: na::Vector2::new(diagonal, -diagonal),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_t_junction_square() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 0), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, true)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(0.0, 0.1),
            right_up: na::Vector2::new(0.0, 0.1),
            left_down: na::Vector2::new(-0.1, -0.1),
            right_down: na::Vector2::new(0.1, -0.1),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_crossroads() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 0), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 2), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, false)[(1, 1)];

        let diagonal = (2.0 as f32).sqrt() * 0.1;

        let expected = Offsets{
            left_up: na::Vector2::new(-diagonal, diagonal),
            right_up: na::Vector2::new(diagonal, diagonal),
            left_down: na::Vector2::new(-diagonal, -diagonal),
            right_down: na::Vector2::new(diagonal, -diagonal),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_offsets_crossroads_square() {
        let rivers = vec![
            River::new(na::Vector2::new(0, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 0), na::Vector2::new(1, 1), 0.1, 0.2),
            River::new(na::Vector2::new(1, 1), na::Vector2::new(1, 2), 0.2, 0.3),
        ];

        let actual = get_offsets(3, 3, &rivers, true)[(1, 1)];

        let expected = Offsets{
            left_up: na::Vector2::new(-0.1, 0.1),
            right_up: na::Vector2::new(0.1, 0.1),
            left_down: na::Vector2::new(-0.1, -0.1),
            right_down: na::Vector2::new(0.1, -0.1),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_points() {
        let heights = na::DMatrix::from_row_slice(3, 3, &[
            90.0, 80.0, 70.0,
            60.0, 50.0, 40.0,
            30.0, 20.0, 100.0
        ]).transpose();

        let mut offsets = na::DMatrix::from_element(3, 3, Offsets::all_zero());

        offsets[(0, 0)].right_up = na::Vector2::new(0.1, -0.1);
        offsets[(1, 0)].left_up = na::Vector2::new(0.2, -0.2);
        offsets[(1, 1)].left_down = na::Vector2::new(0.3, -0.3);
        offsets[(0, 1)].right_down = na::Vector2::new(0.4, -0.4);

        let actual  = get_points(0, 0, &heights, &offsets);

        let expected = [
            na::Vector3::new(0.1, -0.1, 90.0),
            na::Vector3::new(1.2, -0.2, 80.0),
            na::Vector3::new(1.3, -0.7, 50.0),
            na::Vector3::new(0.4, -0.6, 60.0),
        ];
    }

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