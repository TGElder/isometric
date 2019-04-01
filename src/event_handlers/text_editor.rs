// use std::sync::Arc;
// use ::engine::{Event, Command};
// use ::events::EventHandler;
// use ::coords::WorldCoord;
// use ::color::Color;
// use ::graphics::drawing::HouseDrawing;

// const LAYER_NAME: &str = "house";

// pub struct TextEditor {
//     text: String,
// }

// impl TextEditor {

//     fn append(&mut self, character: char, uppercase: bool) {
//         if !uppercase {
//             character = character.to_ascii_lowercase();
//         }
//         self.text.push(character);
//     }

//     fn handle_key(&mut self, keycode: glutin::VirtualKeyCode, modifiers: glutin::ModifiersState) {
//         match keycode {
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::B => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::C => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::D => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::E => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::F => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::G => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::H => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::I => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::J => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::K => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::L => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::M => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::N => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::O => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::P => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::Q => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             glutin::VirtualKeyCode::A => self.append('A', modifiers.shift),
//             _ => {},
//         }
//     }
// }

// impl  EventHandler for TextEditor {
//     fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
//         match *event {
//             Event::WorldPositionChanged(world_coordinate) => {self.world_coordinate = Some(world_coordinate); vec![]},
//             Event::GlutinEvent(
//                 glutin::Event::WindowEvent{
//                     event: glutin::WindowEvent::KeyboardInput{
//                         input: glutin::KeyboardInput{
//                             virtual_keycode: Some(key), 
//                             state: glutin::ElementState::Pressed,
//                             ..
//                         },
//                     ..
//                     },
//                 ..
//                 }
//             ) => {println!("{:?}", key); vec![]},
//             _ => vec![],
//         }
//     }
// }