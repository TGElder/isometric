extern crate image;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use image::open;
use image::GenericImage;

pub mod geometry;

use geometry::*;
use graphics::types;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, lines: &Vec<types::Line>, polygons: &Vec<types::Polygon>, args: &RenderArgs) {

        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c.transform.trans(1024.0, 0.0).scale(4.0, 4.0);
                                       
            // Draw the lines
            for l in lines {
                line(BLACK, 0.2, *l, transform, gl);
            }
            
        });
    }

}

fn main() {

    let img = open("power256full.png").unwrap();
    let width: usize = 256;
    let height: usize = 256;

    let mut screen_coords: Vec<Vec<Coord2D>> = Vec::with_capacity(width);
    for x in 0..width {
        screen_coords.push(Vec::with_capacity(height));
        for _ in 0..height {
            screen_coords[x].push(Coord2D::new(0.0, 0.0));
        }
    }

    println!("{:?}", img.dimensions());

    for (x, y, pixel) in img.pixels() {
        screen_coords[x as usize][y as usize] = Coord3D::new(x as f64, y as f64, pixel.data[0] as f64).to_isometric();
    }

    let xds: [u8; 4] = [0, 1, 1, 0];
    let yds: [u8; 4] = [0, 0, 1, 1];

    use graphics::*;

    let mut lines: Vec<types::Line> = vec![];
    let mut polygons: Vec<types::Polygon> = vec![];

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let mut coord_from = &screen_coords[x][y];
            let mut polygon: [[f64; 2]; 4] = [[0.0, 0.0]; 4];
            for d in 0..4 {
                polygon[d] = [coord_from.x, coord_from.y];
                let xd = x + (xds[d] as usize);
                let yd = y + (yds[d] as usize);
                let coord_to = &screen_coords[xd][yd];
                let line = [coord_from.x, coord_from.y, coord_to.x, coord_to.y];
                lines.push(line);
                coord_from = coord_to;
            }
            polygons.push(&polygon);
        }
    }

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "isometric",
            [2048, 1024]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&lines, &polygons, &r);
        }
    }
}