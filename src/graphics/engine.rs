use super::program::Program;
use std::ffi::c_void;
use std::collections::HashMap;

use ::transform::Transform;
use ::transform::IsometricRotation;
use ::coords::*;
use super::drawing::Drawing;

use ::itertools::Itertools;

#[derive(PartialEq)]
pub enum DrawingType {
    Plain,
    Text
}

pub struct ProgramConfiguration {
    program: Program,
    before_all: Box<Fn(&GraphicsEngine, &Program) -> ()>,
    before_drawing: Box<Fn(&Box<Drawing>, &Program) -> ()>,
}

pub struct GraphicsEngine {
    programs: [ProgramConfiguration; 2],
    viewport_size: glutin::dpi::PhysicalSize,
    transform: Transform,
    drawings: HashMap<String, Box<Drawing>>,
}

impl GraphicsEngine {

    fn get_index(drawing_type: &DrawingType) -> usize {
        match drawing_type {
            DrawingType::Plain => 0,
            DrawingType::Text => 1,
        }
    }

    pub fn new(z_scale: f32, viewport_size: glutin::dpi::PhysicalSize) -> GraphicsEngine {

        let programs = [
            ProgramConfiguration{
                program: Program::from_shaders(include_str!("shaders/triangle.vert"), include_str!("shaders/triangle.frag")),
                before_all: Box::new(|graphics: &GraphicsEngine, program: &Program| {
                    program.load_matrix4("projection", graphics.transform.get_projection_matrix());
                }),
                before_drawing: Box::new(|drawing: &Box<Drawing>, program: &Program| {
                    program.load_float("z_mod", drawing.get_z_mod());
                })
            },
            ProgramConfiguration{
                program: Program::from_shaders(include_str!("shaders/text.vert"), include_str!("shaders/text.frag")),
                before_all: Box::new(|graphics: &GraphicsEngine, program: &Program| {
                    program.load_matrix4("projection", graphics.transform.get_projection_matrix());
                    program.load_matrix2("pixel_to_screen", graphics.get_pixel_to_screen());
                }),
                before_drawing: Box::new(|drawing: &Box<Drawing>, program: & Program| {
                    program.load_float("z_mod", drawing.get_z_mod());
                }),
            }
        ];

        let mut out = GraphicsEngine {
            programs,
            transform: Transform::new(
                GLCoord3D::new(1.0, viewport_size.width as f32 / viewport_size.height as f32, z_scale),
                GLCoord2D::new(0.0, 0.0),
                IsometricRotation::TopLeftAtTop),
            viewport_size,
            drawings: HashMap::new(),
        };
        out.set_viewport_size(viewport_size);
        out.setup_open_gl();
        out
    }

    fn setup_open_gl(&mut self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn get_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_drawing(&mut self, name: String, drawing: Box<Drawing>) {
        self.drawings.insert(name, drawing);
    }

    pub fn remove_drawing(&mut self, name: &String) {
        self.drawings.remove(name);
    }

    fn get_pixel_to_screen(&self) -> na::Matrix2<f32> {
        na::Matrix2::new(
            2.0 / self.viewport_size.width as f32, 0.0,
            0.0, 2.0 / self.viewport_size.height as f32,
        )
    }

    pub fn draw(&mut self) {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.transform.compute_projection_matrix();
            for i in 0..2 {
                let program = &self.programs[i];
                program.program.set_used();
                (program.before_all)(self, &program.program);
                for drawing in self.drawings.values() {
                    if GraphicsEngine::get_index(drawing.drawing_type()) == i {
                        (program.before_drawing)(&drawing, &program.program);
                        drawing.draw();
                    }
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
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
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
