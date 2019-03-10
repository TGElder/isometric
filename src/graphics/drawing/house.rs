use super::super::coords::*;
use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, ColoredVertex};
use super::utils::{SquareColoring, get_colored_vertices_from_square};

pub struct HouseDrawing {
    vbo: VBO<ColoredVertex>,
}

impl Drawing for HouseDrawing {
    fn draw(&self) {
        self.vbo.draw();
    }

    fn get_z_mod(&self) -> f32 {
        0.0
    }
}

impl HouseDrawing {
    pub fn new(world_coordinate: WorldCoord, width: f32, height: f32, roof_height: f32, coloring: Box<SquareColoring>) -> HouseDrawing {

        let x = world_coordinate.x as f32;
        let y = world_coordinate.y as f32;
        let z = world_coordinate.z as f32;

        let a = na::Vector3::new(x - width, y - width, 0.0);
        let b = na::Vector3::new(x + width, y - width, 0.0);
        let c = na::Vector3::new(x + width, y + width, 0.0);
        let d = na::Vector3::new(x - width, y + width, 0.0);
        let e = na::Vector3::new(x - width, y - width, z + height);
        let f = na::Vector3::new(x + width, y - width, z + height);
        let g = na::Vector3::new(x + width, y + width, z + height);
        let h = na::Vector3::new(x - width, y + width, z + height);

        let s = na::Vector3::new(x, y, z + height + roof_height); //TODO

        let mut vbo = VBO::new(gl::TRIANGLES);

        let mut vertices = vec![];
        vertices.append(&mut get_colored_vertices_from_square(&[e, h, d, a], &coloring));
        vertices.append(&mut get_colored_vertices_from_square(&[h, g, c, d], &coloring));
        vertices.append(&mut get_colored_vertices_from_square(&[g, f, b, c], &coloring));
        vertices.append(&mut get_colored_vertices_from_square(&[f, e, a, b], &coloring));
        vertices.append(&mut get_colored_vertices_from_square(&[e, f, g, h], &coloring));

        vbo.load(vertices);

        HouseDrawing{vbo}
    }
}
