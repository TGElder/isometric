use std::sync::Arc;
use ::engine::{Event, Command};
use ::events::EventHandler;
use ::coords::{GLCoord2D, GLCoord4D};

pub struct Scroller{}

impl Scroller{
    pub fn new() -> Scroller{
        Scroller{}
    }
}

impl EventHandler for Scroller{
    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {
        match *event {
            Event::Drag(
                GLCoord4D{x, y, ..}
            ) => vec![Command::Translate(GLCoord2D{x, y})],
            _ => vec![]
        }
    }
}