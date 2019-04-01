extern crate glium;
extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;

use std::sync::Arc;

use ::events::{EventHandler, AsyncEventHandler};
use ::graphics::engine::GraphicsEngine;
use ::event_handlers::*;
use ::transform::Direction;
use ::coords::*;
use ::color::Color;

use ::graphics::drawing::*;

use ::terrain::*;

use glutin::GlContext;

use self::conrod_core::{widget, Colorable, Positionable, Widget};


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
    graphics: GraphicsEngine,
    running: bool,
    events: Vec<Event>,
    event_handlers: Vec<Box<EventHandler>>,
    display: GliumDisplayWinitWrapper,
    ui: conrod_core::Ui,
}

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }
    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

impl IsometricEngine {
    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32, max_z: f32, event_handler: Box<EventHandler>) -> IsometricEngine {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(IsometricEngine::GL_VERSION)
            .with_vsync(true);
        //let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let display = GliumDisplayWinitWrapper(display);
        
        let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let font_path = "NotoSans-Regular.ttf";
        ui.fonts.insert_from_file(font_path).unwrap();

        unsafe {
            display.0.gl_window().make_current().unwrap();
            gl::load_with(|symbol| display.0.gl_window().get_proc_address(symbol) as *const _);
        }

        let dpi_factor = display.0.gl_window().get_hidpi_factor();
        let graphics = GraphicsEngine::new(1.0/max_z, display.0.gl_window().window().get_inner_size().unwrap().to_physical(dpi_factor));

        IsometricEngine {
            events_loop,
            event_handlers: IsometricEngine::init_event_handlers(&display.0, event_handler),
            graphics,
            running: true,
            events: vec![Event::Start],
            display,
            ui,
        }
    }

    fn init_event_handlers(display: &glium::Display, event_handler: Box<EventHandler>) -> Vec<Box<EventHandler>> {
        let dpi_factor = display.gl_window().get_hidpi_factor();
        let logical_window_size = display.gl_window().window().get_inner_size().unwrap();
       
        vec![
            event_handler,
            Box::new(AsyncEventHandler::new(Box::new(ShutdownHandler::new()))),
            Box::new(DPIRelay::new()),
            Box::new(Resizer::new()),
            Box::new(CursorHandler::new(dpi_factor, logical_window_size)),
            Box::new(DragHandler::new()),
            Box::new(ResizeRelay::new(dpi_factor)),
            Box::new(Scroller::new()),
            Box::new(ZoomHandler::new()),
            Box::new(RotateHandler::new()),
            Box::new(HouseBuilder::new(na::Vector3::new(1.0, 0.0, 1.0))),
        ]
    }

    pub fn run(&mut self) {    
        while self.running {
            self.add_glutin_events();
            self.handle_events();

            widget::Text::new("Hello World!")
                .middle_of(self.ui.window)
                .color(conrod_core::color::WHITE)
                .font_size(32)
                .set("id", &mut self.ui.set_widgets());

            if let Some(primitives) = self.ui.draw_if_changed() {

                // The image map describing each of our widget->image mappings (in our case, none).
                let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

                // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
                // for drawing to the glium `Surface`.
                let mut renderer = conrod_glium::Renderer::new(&self.display.0).unwrap();

                renderer.fill(&self.display.0, primitives, &image_map);
                let mut target = self.display.0.draw();
                renderer.draw(&self.display.0, &mut target, &image_map).unwrap();
            }
            


            self.graphics.draw();
            self.display.0.gl_window().swap_buffers().unwrap();
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
            match &event {
                Event::GlutinEvent(event) => {
                    if let Some(event) = conrod_winit::convert_event(event.clone(), &self.display) {
                        self.ui.handle_event(event);
                    }
                },
                _ => {},
            }
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
                self.display.0.gl_window().resize(physical_size);
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
