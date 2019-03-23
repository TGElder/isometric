use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::graphics::coords::WorldCoord;
use ::graphics::engine::Color;
use ::graphics::drawing::house::HouseDrawing;

const LAYER_NAME: &str = "house";

pub struct HouseBuilder {
    houses: usize,
    world_coordinate: Option<WorldCoord>,
    light_direction: na::Vector3<f32>,
}

impl  HouseBuilder {
    pub fn new(light_direction: na::Vector3<f32>) -> HouseBuilder {
        HouseBuilder{
            houses: 0,
            world_coordinate: None,
            light_direction,
        }
    }

    fn draw(&mut self) -> Vec<Command> {
        if let Some(world_coordinate) = self.world_coordinate {
            let world_coordinate = WorldCoord::new(world_coordinate.x.floor() + 0.5, world_coordinate.y.floor() + 0.5, world_coordinate.z);
            let color = Color::new(1.0, 0.0, 0.0, 1.0);
            let drawing = HouseDrawing::new(world_coordinate, 0.25, 0.5, 0.5, color, self.light_direction);
            let name = LAYER_NAME.to_string() + &self.houses.to_string();
            self.houses += 1;
            vec![Command::Draw{name, drawing: Box::new(drawing)}]
        } else {
            vec![]
        }
    }

}

impl  EventHandler for HouseBuilder {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::WorldPositionChanged(world_coordinate) => {self.world_coordinate = Some(world_coordinate); vec![]},
            Event::GlutinEvent(
                glutin::Event::WindowEvent{
                    event: glutin::WindowEvent::KeyboardInput{
                        input: glutin::KeyboardInput{
                            virtual_keycode: Some(glutin::VirtualKeyCode::B), 
                            state: glutin::ElementState::Pressed,
                            ..
                        },
                    ..
                    },
                ..
                }
            ) => self.draw(),
            _ => vec![],
        }
    }
}