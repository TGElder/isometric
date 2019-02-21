extern crate glutin;

use ::graphics::engine::GraphicsEngine;
use ::graphics::transformer::Direction;
use ::graphics::coords::*;

use self::glutin::GlContext;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    graphics: GraphicsEngine,
    drag_controller: DragController,
    terrain: na::DMatrix<f32>,
}

impl IsometricEngine {
    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32, terrain: na::DMatrix<f32>) -> IsometricEngine {
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
        }

        let dpi_factor = gl_window.get_hidpi_factor();
        let mut graphics = GraphicsEngine::new(gl_window.window().get_inner_size().unwrap().to_physical(dpi_factor));
        graphics.load_terrain(&terrain);

        IsometricEngine {
            events_loop,
            window: gl_window,
            graphics,
            drag_controller: DragController::new(),
            terrain,
        }
    }

    pub fn run(&mut self) {
        let mut current_cursor_position = None;
        let mut running = true;
        while running {
            let graphics = &mut self.graphics;
            let events_loop = &mut self.events_loop;
            let window = &self.window;
            let drag_controller = &mut self.drag_controller;
            let dpi_factor = window.get_hidpi_factor();
            let logical_window_size = self.window.window().get_inner_size().unwrap();
            let physical_window_size = logical_window_size.to_physical(dpi_factor);
            let terrain = &self.terrain;
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let physical_size = logical_size.to_physical(dpi_factor);
                            window.resize(physical_size);
                            graphics.set_viewport_size(physical_size);
                        }
                        glutin::WindowEvent::MouseWheel { delta, .. } => {
                            if let Some(cursor_position) = current_cursor_position {
                                match delta {
                                    glutin::MouseScrollDelta::LineDelta(_, d) if d > 0.0 => {
                                        graphics.get_transformer().scale(cursor_position, GLCoord2D{x: 2.0, y: 2.0}) //TODO GLCoords make constant
                                    }
                                    glutin::MouseScrollDelta::LineDelta(_, d) if d < 0.0 => {
                                        graphics.get_transformer().scale(cursor_position, GLCoord2D{x: 0.5, y: 0.5})
                                    }
                                    _ => (),
                                }
                            }
                        },
                        glutin::WindowEvent::KeyboardInput{ input, .. } => match input {
                            glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::Space), 
                                state: glutin::ElementState::Pressed,
                                modifiers, .. } => if let Some(cursor_position) = current_cursor_position {
                                    match modifiers {
                                        glutin::ModifiersState{ shift: true, .. } => graphics.get_transformer().rotate(cursor_position, &Direction::Clockwise),
                                        _ => graphics.get_transformer().rotate(cursor_position, &Direction::AntiClockwise),
                                    }
                            },
                            glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape), 
                                state: glutin::ElementState::Pressed,
                                .. } => running = false,
                            _ => (),
                        },
                        glutin::WindowEvent::CursorMoved{ position, .. } => {
                            let cursor_position = position.to_physical(dpi_factor).to_gl_coord_4d(physical_window_size, graphics);
                            let world_position = cursor_position.to_world_coord(graphics.get_transformer());
                            current_cursor_position = Some(cursor_position);
                            graphics.select_cell(&terrain, na::Point2::new(world_position.x as i32, world_position.y as i32));
                        },
                        _ => (),
                    };
                    if let Some(logical_position) = drag_controller.handle(event) {
                        let physical_delta = logical_position.to_physical(dpi_factor);
                        let gl_delta = GLCoord2D{x: (physical_delta.x / physical_window_size.width) as f32 * -2.0, y: (physical_delta.y / physical_window_size.height) as f32 * 2.0};
                        graphics.get_transformer().translate(gl_delta);
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

    fn handle(&mut self, event: glutin::WindowEvent) -> Option<glutin::dpi::LogicalPosition> {
        match event {
            glutin::WindowEvent::MouseInput{
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
            glutin::WindowEvent::CursorMoved{ position, .. } => {
                let out = if self.dragging {
                    if let Some(last_pos) = self.last_pos {
                        Some(glutin::dpi::LogicalPosition::new(last_pos.x - position.x, last_pos.y - position.y))
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

