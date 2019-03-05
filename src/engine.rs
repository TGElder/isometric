extern crate glutin;

use std::sync::Arc;
use std::collections::HashMap;

use ::events::{EventHandler, AsyncEventHandler};
use ::event_handlers::shutdown::ShutdownHandler;
use ::event_handlers::cursor_handler::CursorHandler;
use ::event_handlers::drag::DragHandler;
use ::event_handlers::scroll::Scroller;
use ::event_handlers::zoom::ZoomHandler;
use ::event_handlers::rotate::RotateHandler;
use ::event_handlers::resize::{ResizeRelay, DPIRelay, Resizer};
use ::event_handlers::selected_cell::SelectedCell;

use ::graphics::engine::{Drawing, GraphicsEngine};
use ::graphics::transform::Direction;
use ::graphics::coords::*;
use ::graphics::drawing::terrain::{TerrainDrawing, TerrainGridDrawing};
use ::graphics::drawing::sea::SeaDrawing;


use self::glutin::GlContext;


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
            terrain,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;
        let mut events = vec![Event::Start];
        let mut drawings = self.init_drawings();
        let mut event_handlers = self.init_event_handlers(&self.terrain);
    
        while running {
            
            self.events_loop.poll_events(|event| {
                events.push(Event::GlutinEvent(event));
            });
            
            let mut next_events: Vec<Event> = vec![];

            let graphics = &mut self.graphics;
            let window = &self.window;
            events.drain(0..).for_each(|event| {
                let event_arc = Arc::new(event);
                for handler in &mut event_handlers {
                    for command in handler.handle_event(event_arc.clone()) {
                        match command {
                            Command::Shutdown => running = false,
                            Command::Resize(physical_size) => {
                                window.resize(physical_size);
                                graphics.set_viewport_size(physical_size);
                            },
                            Command::Translate(translation) => graphics.get_transformer().translate(translation),
                            Command::Scale{center, scale} => graphics.get_transformer().scale(center, scale),
                            Command::Rotate{center, direction} => graphics.get_transformer().rotate(center, direction),    
                            Command::Event(event) => next_events.push(event),
                            Command::ComputeWorldPosition(gl_coord) => next_events.push(Event::WorldPositionChanged(gl_coord.to_world_coord(&graphics.get_transformer()))),
                            Command::Draw{name, drawing} => {drawings.insert(name, drawing);},
                            Command::Erase(name) => {drawings.remove(&name);},
                        }
                    }
                };
            });

            events = next_events;

            let to_draw: Vec<&Drawing> = drawings.values().map(|drawing| drawing.as_ref()).collect();
            graphics.draw(&to_draw);

            self.window.swap_buffers().unwrap();
        }

        for handler in &mut event_handlers {
            handler.handle_event(Arc::new(Event::Shutdown));
        }
    }

    fn init_drawings(&self) -> HashMap<String, Box<Drawing>> {
        let mut drawings: HashMap<String, Box<Drawing>> = HashMap::new();
        drawings.insert("sea".to_string(), Box::new(SeaDrawing::new(self.terrain.shape().0 as f32, self.terrain.shape().1 as f32, 10.0)));
        drawings.insert("terrain".to_string(), Box::new(TerrainDrawing::from_heights(&self.terrain)));
        drawings.insert("terrain_grid".to_string(), Box::new(TerrainGridDrawing::from_heights(&self.terrain)));
        drawings
    }

    fn init_event_handlers <'a> (&self, heights: &'a na::DMatrix<f32>) -> Vec<Box<EventHandler + 'a>> {
        let dpi_factor = self.window.get_hidpi_factor();
        let logical_window_size = self.window.window().get_inner_size().unwrap();
       
        vec![
            Box::new(AsyncEventHandler::new(Box::new(ShutdownHandler::new()))),
            Box::new(DPIRelay::new()),
            Box::new(Resizer::new()),
            Box::new(CursorHandler::new(dpi_factor, logical_window_size)),
            Box::new(DragHandler::new()),
            Box::new(ResizeRelay::new(dpi_factor)),
            Box::new(Scroller::new()),
            Box::new(ZoomHandler::new()),
            Box::new(RotateHandler::new()),
            Box::new(SelectedCell::<'a>::new(heights))
        ]
    }

}