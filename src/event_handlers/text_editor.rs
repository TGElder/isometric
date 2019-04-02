use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;

pub struct TextEditor {
    caret_position: usize,
    text: String,
}

impl TextEditor {

    pub fn new() -> TextEditor {
        TextEditor{caret_position: 0, text: String::new()}
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    fn insert_letter(&mut self, mut character: char, uppercase: bool) {
        if !uppercase {
            character = character.to_ascii_lowercase();
        }
        self.insert_character(character);
    }

    fn advance_caret(&mut self) {
        self.caret_position = (self.caret_position + 1).min(self.text.len());
    }

    fn retreat_caret(&mut self) {
        self.caret_position = (self.caret_position - 1).max(0);
    }

    fn insert_character(&mut self, character: char) {
        self.text.insert(self.caret_position, character);
        self.caret_position += 1;
    }

    fn backspace(&mut self) {
        if self.caret_position > 0 {
            self.text.remove(self.caret_position - 1);
            self.caret_position -= 1;
        }
    }

    fn handle_key(&mut self, keycode: glutin::VirtualKeyCode, modifiers: glutin::ModifiersState) {
        match keycode {
            glutin::VirtualKeyCode::A => self.insert_letter('A', modifiers.shift),
            glutin::VirtualKeyCode::B => self.insert_letter('B', modifiers.shift),
            glutin::VirtualKeyCode::C => self.insert_letter('C', modifiers.shift),
            glutin::VirtualKeyCode::D => self.insert_letter('D', modifiers.shift),
            glutin::VirtualKeyCode::E => self.insert_letter('E', modifiers.shift),
            glutin::VirtualKeyCode::F => self.insert_letter('F', modifiers.shift),
            glutin::VirtualKeyCode::G => self.insert_letter('G', modifiers.shift),
            glutin::VirtualKeyCode::H => self.insert_letter('H', modifiers.shift),
            glutin::VirtualKeyCode::I => self.insert_letter('I', modifiers.shift),
            glutin::VirtualKeyCode::J => self.insert_letter('J', modifiers.shift),
            glutin::VirtualKeyCode::K => self.insert_letter('K', modifiers.shift),
            glutin::VirtualKeyCode::L => self.insert_letter('L', modifiers.shift),
            glutin::VirtualKeyCode::M => self.insert_letter('M', modifiers.shift),
            glutin::VirtualKeyCode::N => self.insert_letter('N', modifiers.shift),
            glutin::VirtualKeyCode::O => self.insert_letter('O', modifiers.shift),
            glutin::VirtualKeyCode::P => self.insert_letter('P', modifiers.shift),
            glutin::VirtualKeyCode::Q => self.insert_letter('Q', modifiers.shift),
            glutin::VirtualKeyCode::R => self.insert_letter('R', modifiers.shift),
            glutin::VirtualKeyCode::S => self.insert_letter('S', modifiers.shift),
            glutin::VirtualKeyCode::T => self.insert_letter('T', modifiers.shift),
            glutin::VirtualKeyCode::U => self.insert_letter('U', modifiers.shift),
            glutin::VirtualKeyCode::V => self.insert_letter('V', modifiers.shift),
            glutin::VirtualKeyCode::W => self.insert_letter('W', modifiers.shift),
            glutin::VirtualKeyCode::X => self.insert_letter('X', modifiers.shift),
            glutin::VirtualKeyCode::Y => self.insert_letter('Y', modifiers.shift),
            glutin::VirtualKeyCode::Z => self.insert_letter('Z', modifiers.shift),
            glutin::VirtualKeyCode::Space => self.insert_character(' '),
            glutin::VirtualKeyCode::Key0 => self.insert_character('0'),
            glutin::VirtualKeyCode::Key1 => self.insert_character('1'),
            glutin::VirtualKeyCode::Key2 => self.insert_character('2'),
            glutin::VirtualKeyCode::Key3 => self.insert_character('3'),
            glutin::VirtualKeyCode::Key4 => self.insert_character('4'),
            glutin::VirtualKeyCode::Key5 => self.insert_character('5'),
            glutin::VirtualKeyCode::Key6 => self.insert_character('6'),
            glutin::VirtualKeyCode::Key7 => self.insert_character('7'),
            glutin::VirtualKeyCode::Key8 => self.insert_character('8'),
            glutin::VirtualKeyCode::Key9 => self.insert_character('9'),
            glutin::VirtualKeyCode::Right => self.advance_caret(),
            glutin::VirtualKeyCode::Left => self.retreat_caret(),
            glutin::VirtualKeyCode::Back => self.backspace(),
            _ => {},
        }
    }
}

impl EventHandler for TextEditor {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::KeyboardInput{
                        input: glutin::KeyboardInput{
                            virtual_keycode: Some(key), 
                            state: glutin::ElementState::Pressed,
                            modifiers,
                            ..
                        },
                    ..
                    },
                ..
                }
            ) => {self.handle_key(key, modifiers); vec![]},
            _ => vec![],
        }
    }
}