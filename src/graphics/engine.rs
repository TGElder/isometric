use super::program::Program;
use std::ffi::c_void;
use std::collections::HashMap;

use ::transform::Transform;
use ::transform::IsometricRotation;
use ::coords::*;
use super::drawing::Drawing;

#[derive(PartialEq)]
pub enum DrawingType {
    Plain,
    Text
}

pub struct GraphicsEngine {
    programs: [Program; 2],
    viewport_size: glutin::dpi::PhysicalSize,
    transform: Transform,
    drawings: HashMap<String, Box<Drawing>>,
}

impl GraphicsEngine {

    pub fn new(z_scale: f32, viewport_size: glutin::dpi::PhysicalSize) -> GraphicsEngine {

        let programs = [
            Program::from_shaders(DrawingType::Plain, include_str!("shaders/triangle.vert"), include_str!("shaders/triangle.frag")),
            Program::from_shaders(DrawingType::Text, include_str!("shaders/text.vert"), include_str!("shaders/text.frag")),
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

    pub fn prepare_program(&self, program: &Program) {
        match program.drawing_type {
            DrawingType::Plain => program.load_matrix4("projection", self.transform.get_projection_matrix()),
            DrawingType::Text => {
                program.load_matrix4("projection", self.transform.get_projection_matrix());
                program.load_matrix2("pixel_to_screen", self.get_pixel_to_screen());
            },
        }
    }

    pub fn prepare_program_for_drawing(&self, program: &Program, drawing: &Box<Drawing>) {
        match program.drawing_type {
            _ => program.load_float("z_mod", drawing.get_z_mod()),
        }
    }

    pub fn draw(&mut self) {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.transform.compute_projection_matrix();
            for program in self.programs.iter() {
                program.set_used();
                self.prepare_program(program);
                for drawing in self.drawings.values() {
                    if *drawing.drawing_type() == program.drawing_type {
                        self.prepare_program_for_drawing(program, drawing);
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
