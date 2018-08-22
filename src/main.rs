extern crate image;
extern crate piston;
extern crate drag_controller;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use drag_controller::{ DragController, Drag };

use image::open;
use image::GenericImage;

pub mod geometry;

use geometry::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, lines: &Vec<[[f64; 4]; 4]>, polygons: &Vec<[[f64; 2]; 4]>, args: &RenderArgs, drag: &[f64; 2], scale: &f64) {

        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c.transform.trans(drag[0], drag[1]).scale(*scale, *scale);
                                       
            // Draw the lines
            for i in 0..lines.len() {
                for j in 0..4 {
                    line(BLACK, 0.2, lines[i][j], transform, gl);
                }
                polygon(GREEN, &polygons[i], transform, gl);
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

    let mut lines: Vec<[[f64; 4]; 4]> = vec![];
    let mut polygons: Vec<[[f64; 2]; 4]> = vec![];

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let mut polygon: [[f64; 2]; 4] = [[0.0, 0.0]; 4];
            for d in 0..4 {
                let xd = x + (xds[d] as usize);
                let yd = y + (yds[d] as usize);
                let coord_focus = &screen_coords[xd][yd];
                polygon[d] = [coord_focus.x, coord_focus.y];
            }
            polygons.push(polygon);
            let mut polygon_lines: [[f64; 4]; 4] = [[0.0, 0.0, 0.0, 0.0]; 4];
            for i in 0..4 {
                let j = (i + 1) % 4;
                polygon_lines[i] = [polygon[i][0], polygon[i][1], polygon[j][0], polygon[j][1]];
            }
            lines.push(polygon_lines);
        }
    }

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "isometric",
            [2048, 1024]
        )
        .opengl(opengl)
        .vsync(false)
        .exit_on_esc(true)
        .samples(0)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut drag_last_pos: [f64; 2] = [0.0, 0.0];
    let mut drag: [f64; 2] = [1024.0, 0.0];
    let mut drag_controller = DragController::new();
    let mut scale: f64 = 0.8;

    while let Some(e) = window.next() {

        if let Some(r) = e.render_args() {
            app.render(&lines, &polygons, &r, &drag, &scale);
        }
        drag_controller.event(&e, |action| {
            match action {
                Drag::Start(x, y) => {
                    drag_last_pos = [x, y];
                    true
                }
                Drag::Move(x, y) => {
                    drag[0] += x - drag_last_pos[0];
                    drag[1] += y - drag_last_pos[1];
                    drag_last_pos = [x, y];
                    true
                }
                Drag::End(_, _) => false,
                Drag::Interrupt => true,
            }
        });
       
        if let Some(s) = e.mouse_scroll_args() {
            if s[1] == 1.0 {
                scale *= 2.0;
            } else if s[1] == -1.0 {
                scale /= 2.0;
            }
        }

        
    }
}