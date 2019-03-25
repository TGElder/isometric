extern crate glutin;

use std::sync::Arc;

use ::events::{EventHandler, AsyncEventHandler};
use ::graphics::engine::GraphicsEngine;
use ::event_handlers::*;
use ::transform::Direction;
use ::coords::*;
use ::color::Color;

use ::graphics::drawing::*;

use ::terrain::*;

use self::glutin::GlContext;

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
    window: glutin::GlWindow,
    graphics: GraphicsEngine,
    running: bool,
    events: Vec<Event>,
    event_handlers: Vec<Box<EventHandler>>,
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
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        }

        let dpi_factor = gl_window.get_hidpi_factor();
        let graphics = GraphicsEngine::new(1.0/max_z, gl_window.window().get_inner_size().unwrap().to_physical(dpi_factor));

        IsometricEngine {
            events_loop,
            event_handlers: IsometricEngine::init_event_handlers(&gl_window, event_handler),
            window: gl_window,
            graphics,
            running: true,
            events: vec![Event::Start],
        }
    }

    fn init_event_handlers(window: &glutin::GlWindow, event_handler: Box<EventHandler>) -> Vec<Box<EventHandler>> {
        let dpi_factor = window.get_hidpi_factor();
        let logical_window_size = window.window().get_inner_size().unwrap();
       
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
            self.graphics.draw();
            self.window.swap_buffers().unwrap();
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
                self.window.resize(physical_size);
                self.graphics.set_viewport_size(physical_size);
            },
            Command::Translate(translation) => self.graphics.get_transformer().translate(translation),
            Command::Scale{center, scale} => self.graphics.get_transformer().scale(center, scale),
            Command::Rotate{center, direction} => self.graphics.get_transformer().rotate(center, direction),    
            Command::Event(event) => self.events.push(event),
            Command::ComputeWorldPosition(gl_coord) => self.events.push(Event::WorldPositionChanged(gl_coord.to_world_coord(&self.graphics.get_transformer()))),
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

pub struct TerrainHandler {
    heights: na::DMatrix<f32>,
    river_nodes: Vec<Node>,
    rivers: Vec<Edge>,
    road_nodes: Vec<Node>,
    roads: Vec<Edge>,
    sea_level: f32,
    world_coord: Option<WorldCoord>,
    terrain: Terrain,
}

impl TerrainHandler {
    pub fn new(heights: na::DMatrix<f32>, river_nodes: Vec<Node>, rivers: Vec<Edge>, sea_level: f32) -> TerrainHandler {
        TerrainHandler{
            sea_level,
            world_coord: None,
            terrain: TerrainHandler::compute_terrain(&heights, &river_nodes, &rivers, &vec![], &vec![]),
            heights,
            river_nodes,
            rivers,
            road_nodes: vec![],
            roads: vec![],
        }
    }
}

impl TerrainHandler {
    fn select_cell(&self) -> Command {
        if let Some(world_coord) = self.world_coord {
            let drawing = SelectedCellDrawing::select_cell(&self.terrain, world_coord);
            match drawing {
                Some(drawing) => Command::Draw{name: "selected_cell".to_string(), drawing: Box::new(drawing)},
                None => Command::Erase("selected_cell".to_string())
            }
        } else {
            Command::Erase("selected_cell".to_string())
        }
    }

    fn compute_terrain(
        heights: &na::DMatrix<f32>, 
        river_nodes: &Vec<Node>, 
        rivers: &Vec<Edge>,
        road_nodes: &Vec<Node>, 
        roads: &Vec<Edge>,
    ) -> Terrain {
        let mut nodes = vec![];
        nodes.extend(river_nodes.iter().cloned());
        nodes.extend(road_nodes.iter().cloned());
        let mut edges = vec![];
        edges.extend(rivers.iter().cloned());
        edges.extend(roads.iter().cloned());
        Terrain::new(&heights, &nodes, &edges)
    }
    
    fn draw_terrain(&mut self) -> Vec<Command> {
        self.terrain = TerrainHandler::compute_terrain(&self.heights, &self.river_nodes, &self.rivers, &self.road_nodes, &self.roads);
        let coloring = Box::new(AngleSquareColoring::new(Color::new(0.0, 1.0, 0.0, 1.0), na::Vector3::new(1.0, 0.0, 1.0)));
        let river_color = Color::new(0.0, 0.0, 1.0, 1.0);
        let road_color = Color::new(0.3, 0.3, 0.3, 1.0);
        vec![
            Command::Draw{name: "sea".to_string(), drawing: Box::new(SeaDrawing::new(self.heights.shape().0 as f32, self.heights.shape().1 as f32, self.sea_level))},
            Command::Draw{name: "tiles".to_string(), drawing: Box::new(TerrainDrawing::new(&self.terrain, coloring))},
            Command::Draw{name: "river".to_string(), drawing: Box::new(EdgeDrawing::new(&self.terrain, &self.rivers,river_color, 0.0))},
            Command::Draw{name: "rivers_nodes".to_string(), drawing: Box::new(NodeDrawing::new(&self.terrain, &self.river_nodes, river_color, 0.0))},
            Command::Draw{name: "road".to_string(), drawing: Box::new(EdgeDrawing::new(&self.terrain, &self.roads,road_color, -0.001))},
            Command::Draw{name: "road_nodes".to_string(), drawing: Box::new(NodeDrawing::new(&self.terrain, &self.road_nodes, road_color, -0.001))},
        ]
    }
}

impl EventHandler for TerrainHandler {

    fn handle_event(&mut self, event: Arc<Event>) -> Vec<Command> {

        let mut out = vec![];
        out.append(
            &mut match *event {
                Event::Start => self.draw_terrain(),
                Event::WorldPositionChanged(world_coord) => {self.world_coord = Some(world_coord); vec![self.select_cell()]},
                Event::GlutinEvent(
                    glutin::Event::WindowEvent{
                        event: glutin::WindowEvent::KeyboardInput{
                            input: glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::R), 
                                state: glutin::ElementState::Pressed,
                                ..
                            },
                        ..
                        },
                    ..
                    }
                ) => {
                    if let Some(world_coord) = self.world_coord {
                        let cell_x = world_coord.x.floor();
                        let cell_y = world_coord.y.floor();
                        let distance_to_left = world_coord.x - cell_x;
                        let distance_to_right = (cell_x + 1.0) - world_coord.x;
                        let distance_to_bottom = world_coord.y - cell_y;
                        let distance_to_top = (cell_y + 1.0) - world_coord.y; 
                        let min = distance_to_left.min(distance_to_right).min(distance_to_top).min(distance_to_bottom);
                        let (from, to) = match min {
                            d if d == distance_to_left => (na::Vector2::new(cell_x as usize, cell_y as usize + 1), na::Vector2::new(cell_x as usize, cell_y as usize)),
                            d if d == distance_to_right => (na::Vector2::new(cell_x as usize + 1, cell_y as usize + 1), na::Vector2::new(cell_x as usize + 1, cell_y as usize)),
                            d if d == distance_to_bottom => (na::Vector2::new(cell_x as usize + 1, cell_y as usize), na::Vector2::new(cell_x as usize, cell_y as usize)),
                            d if d == distance_to_top => (na::Vector2::new(cell_x as usize + 1, cell_y as usize + 1), na::Vector2::new(cell_x as usize, cell_y as usize + 1)),
                            _ => panic!("Should not happen: minimum of four values does not match any of those values"),
                        };
                        let from_z = self.heights[(from.x, from.y)];
                        let to_z = self.heights[(to.x, to.y)];
                        let rise = if from_z > to_z {from_z - to_z} else{to_z - from_z};
                        if rise * 187.5 < 100.0 {
                            if from.x == to.x {
                                self.road_nodes.push(Node::new(from, 0.05, 0.0));
                                self.road_nodes.push(Node::new(to, 0.05, 0.0));
                            } else {
                                self.road_nodes.push(Node::new(from, 0.0, 0.05));
                                self.road_nodes.push(Node::new(to, 0.0, 0.05));
                            }
                            self.roads.push(Edge::new(from, to));
                            self.draw_terrain()
                        } else {
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                }
                _ => vec![],
            }
        );
        out
    }

}