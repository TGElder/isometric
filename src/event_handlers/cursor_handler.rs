use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::graphics::engine::GLZFinder;
use ::graphics::coords::PhysicalPositionExt;

pub struct CursorHandler {
    z_finder: GLZFinder,
    dpi_factor: f64,
    physical_window_size: glutin::dpi::PhysicalSize,
}

impl CursorHandler {
    pub fn new(dpi_factor: f64, logical_window_size: glutin::dpi::LogicalSize) -> CursorHandler {
        CursorHandler{
            z_finder: GLZFinder{},
            dpi_factor,
            physical_window_size: logical_window_size.to_physical(dpi_factor),
        }
    }

    fn handle_move(&self, position: glutin::dpi::LogicalPosition) -> Vec<Command> {
        return vec![
            Command::Event(
                Event::CursorMoved{
                    position: position
                        .to_physical(self.dpi_factor)
                        .to_gl_coord_4d(self.physical_window_size, &self.z_finder)
                }
            )
        ];
    }
}

impl EventHandler for CursorHandler {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::CursorMoved{
                        position,
                        ..
                    },
                    ..
                },
            ) => self.handle_move(position),
            Event::DPIChanged(dpi) => {self.dpi_factor = dpi; vec![]},
            Event::Resize(physical_size) => {self.physical_window_size = physical_size; vec![]}, //TODO too many vec![]s
            _ => vec![]
        }
    }

}