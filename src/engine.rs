extern crate glutin;

use std::sync::Arc;
use ::events::{EventHandler, AsyncEventHandler};
use ::event_handlers::shutdown::ShutdownHandler;
use ::event_handlers::cursor_handler::CursorHandler;
use ::event_handlers::drag::DragHandler;
use ::event_handlers::scroll::Scroller;
use ::event_handlers::zoom::ZoomHandler;
use ::event_handlers::rotate::RotateHandler;
use ::event_handlers::resize::{ResizeRelay, DPIRelay, Resizer};


use ::graphics::engine::{Drawing, GraphicsEngine};
use ::graphics::transform::Direction;
use ::graphics::coords::*;
use ::graphics::drawing::terrain::{TerrainDrawing, TerrainGridDrawing};
use ::graphics::drawing::selected_cell::SelectedCellDrawing;
use ::graphics::drawing::sea::SeaDrawing;

use self::glutin::GlContext;


pub enum Event {
    Shutdown,
    Resize(glutin::dpi::PhysicalSize),
    DPIChanged(f64),
    CursorMoved{position: GLCoord4D},
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
        //let mut current_cursor_position = None;
        let mut running = true;
        let mut events = vec![]; //TODO
        let sea_drawing = SeaDrawing::new(self.terrain.shape().0 as f32, self.terrain.shape().1 as f32, 10.0);
        let terrain_drawing = TerrainDrawing::from_heights(&self.terrain);
        let terrain_grid_drawing = TerrainGridDrawing::from_heights(&self.terrain);
        //let mut selected_cell_drawing = None;
        let dpi_factor = self.window.get_hidpi_factor();
        let logical_window_size = self.window.window().get_inner_size().unwrap();
        let mut event_handlers: Vec<Box<EventHandler>> = vec![
            Box::new(AsyncEventHandler::new(Box::new(ShutdownHandler::new()))),
            Box::new(DPIRelay::new()),
            Box::new(Resizer::new()),
            Box::new(CursorHandler::new(dpi_factor, logical_window_size)),
            Box::new(DragHandler::new()),
            Box::new(ResizeRelay::new(dpi_factor)),
            Box::new(Scroller::new()),
            Box::new(ZoomHandler::new()),
            Box::new(RotateHandler::new()),
        ];
        while running {
            let graphics = &mut self.graphics;
            let events_loop = &mut self.events_loop;
            let window = &self.window;
            let dpi_factor = window.get_hidpi_factor();
            let logical_window_size = self.window.window().get_inner_size().unwrap();
            let physical_window_size = logical_window_size.to_physical(dpi_factor);
            let terrain = &self.terrain;
            events_loop.poll_events(|event| {
                events.push(Event::GlutinEvent(event));
            });
            
            let mut next_events: Vec<Event> = vec![];

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
                        }
                    }
                };
            });

            events = next_events;

            let mut drawings: Vec<&Drawing> = vec![&terrain_drawing, &terrain_grid_drawing, &sea_drawing];
            // if let Some(ref selected_cell_drawing) = selected_cell_drawing {
            //     drawings.push(selected_cell_drawing)
            // }

            graphics.draw(&drawings);

            self.window.swap_buffers().unwrap();
        }

        for handler in &mut event_handlers {
            handler.handle_event(Arc::new(Event::Shutdown));
        }
    }

}