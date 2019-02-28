use super::program::Program;
use super::shader::Shader;
use std::ffi::{CString, c_void};

use super::transform::Transform;
use super::transform::IsometricRotation;
use super::coords::*;

pub struct GraphicsEngine {
    program: Program,
    viewport_size: glutin::dpi::PhysicalSize,
    transform: Transform,
}

impl GraphicsEngine {

    pub fn new(z_scale: f32, viewport_size: glutin::dpi::PhysicalSize) -> GraphicsEngine {
        let program = GraphicsEngine::load_program();

        let mut out = GraphicsEngine {
            program,
            transform: Transform::new(
                GLCoord3D::new(1.0, viewport_size.width as f32 / viewport_size.height as f32, z_scale),
                GLCoord2D::new(0.0, 0.0),
                IsometricRotation::TopLeftAtTop),
            viewport_size,
        };
        out.set_viewport_size(viewport_size);
        out
    }

    pub fn get_transformer(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn load_program() -> Program {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.vert")).unwrap(), //TODO don't like exposing CString
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let fragment_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

        shader_program.set_used();

        return shader_program;
    }

    fn load_z_mod(&self, z_mod: f32) {
        self.program.load_float("z_mod", z_mod);
    }

    fn load_projection_matrix(&self, projection_matrix: na::Matrix4<f32>) {
        self.program.load_matrix("projection", projection_matrix);
    }

    pub fn draw(&mut self, drawings: &Vec<&Drawing>) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.transform.compute_projection_matrix();
            self.load_projection_matrix(self.transform.get_projection_matrix());
            for drawing in drawings {
                self.load_z_mod(drawing.get_z_mod());
                drawing.draw();
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
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }
    }

}

pub trait Drawing {
    fn draw(&self);
    fn get_z_mod(&self) -> f32;
}

impl ZFinder for GraphicsEngine {

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
