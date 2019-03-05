use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::graphics::coords::GLCoord4D;
use ::graphics::transform::Direction;

pub struct RotateHandler {
    cursor_position: Option<GLCoord4D>,
}

impl RotateHandler {
    pub fn new() -> RotateHandler {
        RotateHandler{
            cursor_position: None
        }
    }

    fn handle_modifiers(&self, modifiers: glutin::ModifiersState) -> Vec<Command> {
        if let Some(center) = self.cursor_position {
            match modifiers {
                glutin::ModifiersState{ 
                    shift: true, 
                    ..
                } => vec![Command::Rotate{center, direction: Direction::Clockwise}],
                _ => vec![Command::Rotate{center, direction: Direction::AntiClockwise}],
            }
        } else {
            vec![]
        }
    }
}

impl EventHandler for RotateHandler {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::KeyboardInput{
                        input: glutin::KeyboardInput{
                            virtual_keycode: Some(glutin::VirtualKeyCode::Space), 
                            state: glutin::ElementState::Pressed,
                            modifiers,
                            ..
                        },
                    ..
                    },
                ..
                }
            ) => self.handle_modifiers(modifiers),
            Event::CursorMoved(gl_position) => {self.cursor_position = Some(gl_position); vec![]},
            _ => vec![]
        }
    }

}