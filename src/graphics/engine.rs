use super::program::Program;
use super::shader::Shader;
use std::ffi::{CString, c_void};
use std::collections::HashMap;

use ::transform::Transform;
use ::transform::IsometricRotation;
use ::coords::*;
use super::drawing::Drawing;

pub struct GraphicsEngine {
    untextured_program: Program,
    text_program: Program,
    viewport_size: glutin::dpi::PhysicalSize,
    transform: Transform,
    drawings: HashMap<String, Box<Drawing>>,
}

impl GraphicsEngine {

    pub fn new(z_scale: f32, viewport_size: glutin::dpi::PhysicalSize) -> GraphicsEngine {
        let untextured_program = GraphicsEngine::load_program(include_str!("shaders/triangle.vert"), include_str!("shaders/triangle.frag"));
        let text_program = GraphicsEngine::load_program(include_str!("shaders/texture.vert"), include_str!("shaders/texture.frag"));

        let mut out = GraphicsEngine {
            untextured_program,
            text_program,
            transform: Transform::new(
                GLCoord3D::new(1.0, viewport_size.width as f32 / viewport_size.height as f32, z_scale),
                GLCoord2D::new(0.0, 0.0),
                IsometricRotation::TopLeftAtTop),
            viewport_size,
            drawings: HashMap::new(),
        };
        out.set_viewport_size(viewport_size);
        out
    }

    pub fn get_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn load_program(vertex_shader: &'static str, fragment_shader: &'static str) -> Program {
        let vertex_shader = Shader::from_source(
            &CString::new(vertex_shader).unwrap(), //TODO don't like exposing CString
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let fragment_shader = Shader::from_source(
            &CString::new(fragment_shader).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

        return shader_program;
    }

    pub fn add_drawing(&mut self, name: String, drawing: Box<Drawing>) {
        self.drawings.insert(name, drawing);
    }

    pub fn remove_drawing(&mut self, name: &String) {
        self.drawings.remove(name);
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.transform.compute_projection_matrix();
            self.untextured_program.set_used();
            self.untextured_program.load_matrix("projection", self.transform.get_projection_matrix());
            for drawing in self.drawings.values() {
                if !drawing.text() {
                    self.untextured_program.load_float("z_mod", drawing.get_z_mod());
                    drawing.draw();
                }
            }
            self.text_program.set_used();
            self.text_program.load_matrix("projection", self.transform.get_projection_matrix());
            for drawing in self.drawings.values() {
                if drawing.text() {
                    self.text_program.load_float("z_mod", drawing.get_z_mod());
                    drawing.draw();
                }
            }
        }
    }

    pub fn set_viewport_size(&mut self, viewport_size: glutin::dpi::PhysicalSize) {
        self.transform.scale(GLCoord4D::new(0.0, 0.0, 0.0, 1.0),
            GLCoord2D::new(
                (self.viewport_size.width as f32) / (viewport_size.width as f32),
                (self.viewport_size.height as f32) / (viewport_size.height as f32)
            )
        );
        self.viewport_size = viewport_size;
        unsafe {
            gl::Viewport(0, 0, viewport_size.width as i32, viewport_size.height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
    }

}

pub struct GLZFinder {}

impl ZFinder for GLZFinder {

    fn get_z_at(&self, buffer_coordinate: BufferCoordinate) -> f32 {
        let mut buffer: Vec<f32> = vec![0.0];
        unsafe {
            gl::ReadPixels(
                buffer_coordinate.x,
                buffer_coordinate.y,
                1,
                1,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                buffer.as_mut_ptr() as *mut c_void
            );
        }
        2.0 * buffer[0] - 1.0
    }
}
