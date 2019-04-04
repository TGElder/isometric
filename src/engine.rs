use std::sync::Arc;

use ::events::{EventHandler, AsyncEventHandler};
use ::graphics::engine::GraphicsEngine;
use ::event_handlers::*;
use ::transform::Direction;
use ::coords::*;
use ::graphics::drawing::*;

use glutin::GlContext;

pub enum Event {
    Start,
    Shutdown,
    Resize(glutin::dpi::PhysicalSize),
    DPIChanged(f64),
    CursorMoved(GLCoord4D),
    WorldPositionChanged(WorldCoord),
    GlutinEvent(glutin::Event),
    Drag(GLCoord4D),
}

pub enum Command {
    Shutdown,
    Resize(glutin::dpi::PhysicalSize),
    Translate(GLCoord2D),
    Scale{center: GLCoord4D, scale: GLCoord2D},
    Rotate{center: GLCoord4D, direction: Direction},
    Event(Event),
    ComputeWorldPosition(GLCoord4D),
    Draw{name: String, drawing: Box<Drawing + Send>},
    Erase(String),
}

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    graphics: GraphicsEngine,
    running: bool,
    events: Vec<Event>,
    event_handlers: Vec<Box<EventHandler>>,
}

impl IsometricEngine {
    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32, max_z: f32) -> IsometricEngine {
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
            event_handlers: IsometricEngine::init_event_handlers(&gl_window),
            window: gl_window,
            graphics,
            running: true,
            events: vec![Event::Start],
        }
    }

    pub fn add_event_handler(&mut self, event_handler: Box<EventHandler>) {
        self.event_handlers.push(event_handler);
    }

    fn init_event_handlers(window: &glutin::GlWindow) -> Vec<Box<EventHandler>> {
        let dpi_factor = window.get_hidpi_factor();
        let logical_window_size = window.window().get_inner_size().unwrap();
       
        vec![
            Box::new(AsyncEventHandler::new(Box::new(ShutdownHandler::new()))),
            Box::new(DPIRelay::new()),
            Box::new(Resizer::new()),
            Box::new(CursorHandler::new(dpi_factor, logical_window_size)),
            Box::new(DragHandler::new()),
            Box::new(ResizeRelay::new(dpi_factor)),
            Box::new(Scroller::new()),
            Box::new(ZoomHandler::new()),
        ]
    }

    pub fn run(&mut self) {    
        while self.running {
            self.add_glutin_events();
            self.handle_events();
            self.graphics.draw();
            self.window.swap_buffers().unwrap();
        }

        self.shutdown();
    }

    fn add_glutin_events(&mut self) {
        let mut glutin_events = vec![];
        self.events_loop.poll_events(|event| {
            glutin_events.push(Event::GlutinEvent(event));
        });
        self.events.append(&mut glutin_events);
    }

    fn handle_events(&mut self) {
        let mut events: Vec<Event> = vec![];
        events.append(&mut self.events);

        let mut commands = vec![];

        events.drain(0..).for_each(|event| {
            let event_arc = Arc::new(event);
            for handler in self.event_handlers.iter_mut() {
                commands.append(&mut handler.handle_event(event_arc.clone()));
            };
        });

        for command in commands {
            self.handle_command(command);
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Shutdown => self.running = false,
            Command::Resize(physical_size) => {
                self.window.resize(physical_size);
                self.graphics.set_viewport_size(physical_size);
            },
            Command::Translate(translation) => self.graphics.get_transform().translate(translation),
            Command::Scale{center, scale} => self.graphics.get_transform().scale(center, scale),
            Command::Rotate{center, direction} => self.graphics.get_transform().rotate(center, direction),    
            Command::Event(event) => self.events.push(event),
            Command::ComputeWorldPosition(gl_coord) => self.events.push(Event::WorldPositionChanged(gl_coord.to_world_coord(&self.graphics.get_transform()))),
            Command::Draw{name, drawing} => {self.graphics.add_drawing(name, drawing)},
            Command::Erase(name) => {self.graphics.remove_drawing(&name)},
        }
    }
    
    fn shutdown(&mut self) {
        for handler in &mut self.event_handlers {
            handler.handle_event(Arc::new(Event::Shutdown));
        }
    }


}
