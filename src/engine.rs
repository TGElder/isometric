extern crate glutin;

use self::glutin::GlContext;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
}

impl IsometricEngine {

    pub fn new(title: &str, width: f64, height: f64) -> IsometricEngine {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let context = glutin::ContextBuilder::new()
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        }


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
}