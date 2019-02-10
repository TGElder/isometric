extern crate glutin;

use ::graphics::engine::GraphicsEngine;

use self::glutin::GlContext;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    graphics: GraphicsEngine,
    drag_controller: DragController,
}

impl IsometricEngine {
    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32, triangle_vertices: Vec<f32>, line_vertices: Vec<f32>) -> IsometricEngine {
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
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }

        let graphics = GraphicsEngine::new(triangle_vertices, line_vertices);

        IsometricEngine {
            events_loop,
            window: gl_window,
            graphics,
            drag_controller: DragController::new(),
        }
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            let graphics = &mut self.graphics;
            let events_loop = &mut self.events_loop;
            let window = &self.window;
            let drag_controller = &mut self.drag_controller;
            let window_size = self.window.window().get_inner_size().unwrap();
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                        }
                        glutin::WindowEvent::MouseWheel { delta, .. } => match delta {
                            glutin::MouseScrollDelta::LineDelta(_, d) if d > 0.0 => {
                                graphics.scale(2.0)
                            }
                            glutin::MouseScrollDelta::LineDelta(_, d) if d < 0.0 => {
                                graphics.scale(0.5)
                            }
                            _ => (),
                        },
                        glutin::WindowEvent::KeyboardInput{ input, .. } => match input {
                            glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::Space), 
                                state: glutin::ElementState::Pressed,
                                modifiers, .. } => match modifiers {
                                    glutin::ModifiersState{ shift: true, .. } => graphics.rotate(3),
                                    _ => graphics.rotate(1),
                            },
                            _ => (),
                        }
                        _ => (),
                    };
                    if let Some((x, y)) = drag_controller.handle(event) {
                        graphics.translate((
                            (x / (window_size.width / 2.0)) as f32,
                            (y / (window_size.height / 2.0)) as f32,
                        ));
                    }
                }
                _ => (),
            });

            graphics.draw();

            self.window.swap_buffers().unwrap();
        }
    }
}

pub struct DragController {
    dragging: bool,
    last_pos: Option<glutin::dpi::LogicalPosition>,
}

impl DragController {
    fn new() -> DragController {
        DragController {
            dragging: false,
            last_pos: None,
        }
    }

    fn handle(&mut self, event: glutin::WindowEvent) -> Option<(f64, f64)> {
        match event {
            glutin::WindowEvent::MouseInput {
                state,
                button: glutin::MouseButton::Left,
                ..
            } => {
                match state {
                    glutin::ElementState::Pressed => self.dragging = true,
                    glutin::ElementState::Released => self.dragging = false,
                };
                None
            }
            glutin::WindowEvent::CursorMoved { position, .. } => {
                let out = if self.dragging {
                    if let Some(last_pos) = self.last_pos {
                        Some((last_pos.x - position.x, last_pos.y - position.y))
                    } else {
                        None
                    }
                } else {
                    None
                };
                self.last_pos = Some(position);
                out
            }
            _ => None,
        }
    }
}