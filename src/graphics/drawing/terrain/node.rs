// use super::super::super::coords::*;
// use super::super::super::engine::{Color, Drawing};
// use super::super::super::vertex_objects::{VBO, ColoredVertex};
// use super::super::utils::*;
// use ::terrain::Terrain;

// pub struct NodeDrawing {
//     vbo: VBO<ColoredVertex>,
// }

// impl Drawing for NodeDrawing {
//     fn draw(&self) {
//         self.vbo.draw();
//     }

//     fn get_z_mod(&self) -> f32 {
//         0.0
//     }
// }

// impl NodeDrawing {
//     pub fn new(
//         terrain: Terrain,
//         base_color: Color,
//         ) -> NodeDrawing {

//         for 

//         let triangle_coloring: Box<TriangleColoring> = Box::new(AngleTriangleColoring::new(base_color, light_direction));
//         let square_coloring: Box<SquareColoring> = Box::new(AngleSquareColoring::new(base_color, light_direction));

//         let x = world_coordinate.x as f32;
//         let y = world_coordinate.y as f32;
//         let z = world_coordinate.z as f32;

//         let a = na::Vector3::new(x - width, y - width, 0.0);
//         let b = na::Vector3::new(x + width, y - width, 0.0);
//         let c = na::Vector3::new(x + width, y + width, 0.0);
//         let d = na::Vector3::new(x - width, y + width, 0.0);
//         let e = na::Vector3::new(x - width, y - width, z + height);
//         let f = na::Vector3::new(x + width, y - width, z + height);
//         let g = na::Vector3::new(x + width, y + width, z + height);
//         let h = na::Vector3::new(x - width, y + width, z + height);

//         let s = na::Vector3::new(x, y, z + height + roof_height);

//         let mut vbo = VBO::new(gl::TRIANGLES);

//         let mut vertices = vec![];
//         vertices.append(&mut get_colored_vertices_from_square(&[e, h, d, a], &square_coloring));
//         vertices.append(&mut get_colored_vertices_from_square(&[h, g, c, d], &square_coloring));
//         vertices.append(&mut get_colored_vertices_from_square(&[g, f, b, c], &square_coloring));
//         vertices.append(&mut get_colored_vertices_from_square(&[f, e, a, b], &square_coloring));
//         vertices.append(&mut get_colored_vertices_from_triangle(&[h, e, s], &triangle_coloring));
//         vertices.append(&mut get_colored_vertices_from_triangle(&[g, h, s], &triangle_coloring));
//         vertices.append(&mut get_colored_vertices_from_triangle(&[f, g, s], &triangle_coloring));
//         vertices.append(&mut get_colored_vertices_from_triangle(&[e, f, s], &triangle_coloring));
        
//         vbo.load(vertices);

//         HouseDrawing{vbo}
//     }
// }
