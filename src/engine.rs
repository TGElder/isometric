extern crate glutin;

use ::graphics::engine::{Drawing, GraphicsEngine};
use ::graphics::transform::Direction;
use ::graphics::coords::*;
use ::graphics::drawing::terrain::{TerrainDrawing, TerrainGridDrawing};
use ::graphics::drawing::selected_cell::SelectedCellDrawing;
use ::graphics::drawing::sea::SeaDrawing;

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

    pub fn new(title: &str, width: u32, height: u32, terrain: na::DMatrix<f32>, max_z: f32) -> IsometricEngine {
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
        let graphics = GraphicsEngine::new(1.0/max_z, gl_window.window().get_inner_size().unwrap().to_physical(dpi_factor));

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
        let sea_drawing = SeaDrawing::new(self.terrain.shape().0 as f32, self.terrain.shape().1 as f32, 10.0);
        let terrain_drawing = TerrainDrawing::from_heights(&self.terrain);
        let terrain_grid_drawing = TerrainGridDrawing::from_heights(&self.terrain);
        let mut selected_cell_drawing = None;
        let mut event_handlers: Vec<Box<EventHandler>> = vec![Box::new(ShutdownHandler::new())];
        while running {
            let graphics = &mut self.graphics;
            let events_loop = &mut self.events_loop;
            let window = &self.window;
            let drag_controller = &mut self.drag_controller;
            let dpi_factor = window.get_hidpi_factor();
            let logical_window_size = self.window.window().get_inner_size().unwrap();
            let physical_window_size = logical_window_size.to_physical(dpi_factor);
            let terrain = &self.terrain;
            events_loop.poll_events(|ref event| {
                match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::Resized(logical_size) => {
                            let physical_size = logical_size.to_physical(dpi_factor);
                            window.resize(physical_size);
                            graphics.set_viewport_size(physical_size);
                        }
                        glutin::WindowEvent::MouseWheel { delta, .. } => {
                            if let Some(cursor_position) = current_cursor_position {
                                match delta {
                                    glutin::MouseScrollDelta::LineDelta(_, d) if d > &0.0 => {
                                        graphics.get_transformer().scale(cursor_position, GLCoord2D{x: 2.0, y: 2.0}) //TODO GLCoords make constant
                                    }
                                    glutin::MouseScrollDelta::LineDelta(_, d) if d < &0.0 => {
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
                            }
                            _ => (),
                        },
                        glutin::WindowEvent::CursorMoved{ position, .. } => {
                            let cursor_position = position.to_physical(dpi_factor).to_gl_coord_4d(physical_window_size, graphics);
                            let world_coordinate = cursor_position.to_world_coord(graphics.get_transformer());
                            current_cursor_position = Some(cursor_position);
                            selected_cell_drawing = SelectedCellDrawing::select_cell(terrain, world_coordinate);
                        },
                        _ => (),
                    };
                    if let Some(gl_delta) = drag_controller.handle(event, dpi_factor, physical_window_size) {
                        graphics.get_transformer().translate(gl_delta);
                    }
                }
                _ => (),
            };
            for handler in &mut event_handlers {
                handler.handle_event(Event::GlutinEvent{glutin_event: &event});
            }
            });

            for handler in &mut event_handlers {
                if let Some(commands) = handler.get_commands() {
                    for command in commands {
                        match command {
                            Command::Shutdown => running = false,
                        }
                    }
                }
            }

            let mut drawings: Vec<&Drawing> = vec![&terrain_drawing, &terrain_grid_drawing, &sea_drawing];
            if let Some(ref selected_cell_drawing) = selected_cell_drawing {
                drawings.push(selected_cell_drawing)
            }

            graphics.draw(&drawings);

            self.window.swap_buffers().unwrap();
        }
    }
}

pub enum Event<'a> {
    GlutinEvent{glutin_event: &'a glutin::Event},
}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
    fn get_commands(&mut self) -> Option<Vec<Command>>;
}

pub enum Command {
    Shutdown,
}

pub struct ShutdownHandler {
    shutdown: bool,
}

impl ShutdownHandler {
    pub fn new() -> ShutdownHandler {
        ShutdownHandler{shutdown: false}
    }
}

impl EventHandler for ShutdownHandler {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::GlutinEvent{glutin_event: glutin::Event::WindowEvent { event: glutin::WindowEvent::CloseRequested, ..}} => self.shutdown = true,
            Event::GlutinEvent{glutin_event: glutin::Event::WindowEvent { event: glutin::WindowEvent::KeyboardInput{ input: glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape), 
                                state: glutin::ElementState::Pressed,
                                .. }, ..}, ..}} => self.shutdown = true,
            _ => (),
        }

    }
    fn get_commands(&mut self) -> Option<Vec<Command>> {
        match self.shutdown {
            true => Some(vec![Command::Shutdown]),
            false => None,
        }
    }
}



pub struct DragController {
    dragging: bool,
    last_pos: Option<GLCoord2D>,
}

impl DragController {
    fn new() -> DragController {
        DragController {
            dragging: false,
            last_pos: None,
        }
    }

    fn handle(&mut self, event: &glutin::WindowEvent, dpi_factor: f64, physical_window_size: glutin::dpi::PhysicalSize) -> Option<GLCoord2D> {
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
                let position = position.to_physical(dpi_factor).to_gl_coord_2d(physical_window_size);
                let out = if self.dragging {
                    if let Some(ref last_pos) = self.last_pos {
                        Some(GLCoord2D{x: position.x - last_pos.x, y: position.y - last_pos.y})
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

