use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::graphics::engine::GLZFinder;
use ::graphics::coords::PhysicalPositionExt;

pub struct CursorHandler {
    z_finder: GLZFinder,
    dpi_factor: Option<f64>,
    physical_window_size: Option<glutin::dpi::PhysicalSize>,
}

impl CursorHandler {
    pub fn new() -> CursorHandler {
        CursorHandler{
            z_finder: GLZFinder{},
            dpi_factor: Some(1.0),
            physical_window_size: None,
        }
    }

    fn handle_move(&self, position: glutin::dpi::LogicalPosition) -> Vec<Command> {
        if let Some(dpi_factor) = self.dpi_factor {
            if let Some(physical_window_size) = self.physical_window_size {
                return vec![
                    Command::Event(
                        Event::CursorMoved{
                            position: position
                                .to_physical(dpi_factor)
                                .to_gl_coord_4d(physical_window_size, &self.z_finder)
                        }
                    )
                ];
            };
        };
        vec![]
    }
}

impl EventHandler for CursorHandler {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::GlutinEvent{
                glutin_event: glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::CursorMoved{
                        position,
                        ..
                    },
                    ..
                },
            } => self.handle_move(position),
            Event::GlutinEvent{
                glutin_event: glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::HiDpiFactorChanged(dpi_factor),
                    ..
                }
            } => {self.dpi_factor = Some(dpi_factor); vec![]}, //TODO too many vec![]s
            Event::Resize{physical_size} => {self.physical_window_size = Some(physical_size); vec![]},
            _ => vec![]
        }
    }

}