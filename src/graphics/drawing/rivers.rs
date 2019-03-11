use super::super::coords::*;
use super::super::engine::{Color, Drawing};
use super::super::vertex_objects::{VBO, ColoredVertex};
use super::utils::*;

pub struct River {
    from: na::Vector2<usize>,
    to: na::Vector2<usize>,
}

impl River {
    pub fn new(from: na::Vector2<usize>, to: na::Vector2<usize>) -> River {
        River{from, to}
    }
}

pub struct RiversDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for RiversDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        -0.0003
    }
}

impl RiversDrawing {
    pub fn new(
        rivers: &Vec<River>,
        heights: &na::DMatrix<f32>,
    ) -> RiversDrawing {

        let width = 0.25;
        let color = Color::new(0.0, 0.0, 1.0, 1.0);

        let mut vertices = vec![];

        for river in rivers {
            let points = if river.from.x == river.to.x {
                let x_minus = river.from.x as f32 - width;
                let x_plus = river.from.x as f32 + width;
                [
                    na::Vector3::new(x_minus, river.from.y as f32, heights[(river.from.x, river.from.y)]),
                    na::Vector3::new(x_plus, river.from.y as f32, heights[(river.from.x, river.from.y)]),
                    na::Vector3::new(x_plus, river.to.y as f32, heights[(river.to.x, river.to.y)]),
                    na::Vector3::new(x_minus, river.to.y as f32, heights[(river.to.x, river.to.y)]),
                ]
            } else {
                let y_minus = river.from.y as f32 - width;
                let y_plus = river.from.y as f32 + width;
                [
                    na::Vector3::new(river.from.x as f32, y_minus, heights[(river.from.x, river.from.y)]),
                    na::Vector3::new(river.from.x as f32, y_plus, heights[(river.from.x, river.from.y)]),
                    na::Vector3::new(river.to.x as f32, y_plus, heights[(river.to.x, river.to.y)]),
                    na::Vector3::new(river.to.x as f32, y_minus, heights[(river.to.x, river.to.y)]),
                ]
            };

            vertices.append(&mut vec![
                points[0].x, points[0].y, points[0].z, color.r, color.g, color.b,
                points[3].x, points[3].y, points[3].z, color.r, color.g, color.b,
                points[2].x, points[2].y, points[2].z, color.r, color.g, color.b,
                points[0].x, points[0].y, points[0].z, color.r, color.g, color.b,
                points[2].x, points[2].y, points[2].z, color.r, color.g, color.b,
                points[1].x, points[1].y, points[1].z, color.r, color.g, color.b,
            ]);
        }

        let mut vbo = VBO::new(gl::TRIANGLES);
        vbo.load(vertices);
        RiversDrawing{vbo}
    }
}

