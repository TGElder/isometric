extern crate glutin;

use self::glutin::GlContext;
use std::ffi::CString;
use shader::Shader;
use program::Program;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
}

impl IsometricEngine {

    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 5));

    pub fn new(title: &str, width: u32, height: u32) -> IsometricEngine {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(IsometricEngine::GL_VERSION)
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        }

        IsometricEngine::load_program();

        IsometricEngine{
            events_loop,
            window: gl_window,
        }
    }

    pub fn run(&mut self) {
        let mut events_loop = &mut self.events_loop;
        let window = &self.window;
        let mut running = true;
        while running {
            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                        },
                        _ => ()
                    },
                    _ => ()
                }
            });

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.swap_buffers().unwrap();
        }
    }

    pub fn load_program() {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.vert")).unwrap(), //TODO don't like exposing CString
            gl::VERTEX_SHADER
        ).unwrap();

        let fragment_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
            gl::FRAGMENT_SHADER
        ).unwrap();

        let shader_program = Program::from_shaders(
            &[vertex_shader, fragment_shader]
        ).unwrap();

        shader_program.set_used();
    }
}

