use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::graphics::coords::WorldCoord;
use ::graphics::drawing::selected_cell::SelectedCellDrawing;

const LAYER_NAME: &str = "selected_cell";

pub struct SelectedCell<'a> {
    heights: &'a na::DMatrix<f32>,
}

impl <'a> SelectedCell<'a> {
    pub fn new(heights: &'a na::DMatrix<f32>) -> SelectedCell {
        SelectedCell{
            heights
        }
    }

    fn draw(&self, world_coordinate: WorldCoord) -> Vec<Command> {
        let drawing = SelectedCellDrawing::select_cell(self.heights, world_coordinate);
        let name = LAYER_NAME.to_string();
        match drawing {
            Some(drawing) => vec![Command::Draw{name, drawing: Box::new(drawing)}],
            None => vec![Command::Erase(name)]
        }
    }
}

impl <'a> EventHandler for SelectedCell<'a> {
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::WorldPositionChanged(world_coordinate) => self.draw(world_coordinate),
            _ => vec![],
        }
    }
}