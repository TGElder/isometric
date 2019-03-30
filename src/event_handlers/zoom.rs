use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::coords::{GLCoord2D, GLCoord4D};

pub struct ZoomHandler {
    cursor_position: Option<GLCoord4D>,
}

impl ZoomHandler {
    pub fn new() -> ZoomHandler {
        ZoomHandler{
            cursor_position: None
        }
    }

    fn zoom_in(&self) -> Vec<Command> {
        if let Some(center) = self.cursor_position {
            vec![Command::Scale{center, scale: GLCoord2D::new(2.0, 2.0)}]
        } else {
            vec![]
        }
    }

    fn zoom_out(&self) -> Vec<Command> {
        if let Some(center) = self.cursor_position {
            vec![Command::Scale{center, scale: GLCoord2D::new(0.5, 0.5)}]
        } else {
            vec![]
        }
    }

    fn handle_mouse_scroll_delta(&self, delta: glutin::MouseScrollDelta) -> Vec<Command> {
        if let Some(center) = self.cursor_position {
            match delta {
                glutin::MouseScrollDelta::LineDelta(_, d) if d > 0.0 => self.zoom_in(),
                glutin::MouseScrollDelta::LineDelta(_, d) if d < 0.0 => self.zoom_out(),
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}

impl EventHandler for ZoomHandler {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::MouseWheel{
                        delta,
                        ..
                    },
                    ..
                },
            ) => self.handle_mouse_scroll_delta(delta),
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::KeyboardInput{
                        input: glutin::KeyboardInput{
                            virtual_keycode, 
                            state: glutin::ElementState::Pressed,
                            ..
                        },
                    ..
                    },
                ..
                }
            ) => match virtual_keycode {
                Some(glutin::VirtualKeyCode::Add) => self.zoom_in(),
                Some(glutin::VirtualKeyCode::Subtract) => self.zoom_out(),
                _ => vec![]
            }
            Event::CursorMoved(gl_position) => {self.cursor_position = Some(gl_position); vec![]},
            _ => vec![]
        }
    }

}