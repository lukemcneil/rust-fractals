extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use rayon::prelude::*;
use std::time::Instant;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent};
use piston::window::WindowSettings;

#[derive(Debug)]
enum Fractal {
    Mandlebrot,
    Julia,
    BurningShip,
}

pub struct App {
    gl: GlGraphics,
    pixel_size: f64,
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    iterations: i32,
    pixel_data: Vec<Vec<Option<[f32; 4]>>>,
    window_width: i32,
    window_height: i32,
    coloring_constant: f32,
    fractal: Fractal,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.pixel_size);
        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for (x, col) in self.pixel_data.iter().enumerate() {
                for (y, &e) in col.iter().enumerate() {
                    if let Some(color) = e {
                        let transform = c
                            .transform
                            .trans(x as f64 * self.pixel_size, y as f64 * self.pixel_size);
                        rectangle(color, square, transform, gl);
                    }
                }
            }
        });
    }

    fn handle_button(&mut self, args: &ButtonArgs) {
        if let ButtonState::Press = args.state {
            if let Button::Keyboard(key) = args.button {
                let mut need_to_update = true;
                match key {
                    Key::W | Key::Up => self.pan(0.0, 1.0),
                    Key::A | Key::Left => self.pan(-1.0, 0.0),
                    Key::S | Key::Down => self.pan(0.0, -1.0),
                    Key::D | Key::Right => self.pan(1.0, 0.0),
                    Key::I => self.zoom(true),
                    Key::O => self.zoom(false),
                    Key::Equals => self.iterations += 100,
                    Key::Minus => {
                        if self.iterations > 100 {
                            self.iterations -= 100
                        }
                    }
                    Key::RightBracket => {
                        if self.pixel_size >= 2.0 {
                            self.pixel_size -= 1.0;
                            self.reset_pixel_data_vecs();
                        }
                    }
                    Key::LeftBracket => {
                        self.pixel_size += 1.0;
                        self.reset_pixel_data_vecs();
                    }
                    Key::Period => self.coloring_constant *= 1.1,
                    Key::Comma => self.coloring_constant /= 1.1,
                    Key::D1 => self.fractal = Fractal::Mandlebrot,
                    Key::D2 => self.fractal = Fractal::Julia,
                    Key::D3 => self.fractal = Fractal::BurningShip,
                    _ => need_to_update = false,
                }
                if need_to_update {
                    self.update_pixel_data();
                }
            }
        }
    }

    fn reset_pixel_data_vecs(&mut self) {
        let new_width = self.window_width / self.pixel_size as i32;
        let new_height = self.window_height / self.pixel_size as i32;
        self.pixel_data.resize(new_width as usize, vec![]);
        for col in &mut self.pixel_data {
            col.resize(new_height as usize, None);
        }
    }

    fn pan(&mut self, x_dir: f64, y_dir: f64) {
        let x_change = (self.x_end - self.x_start) * 0.1 * (x_dir as f64);
        let y_change = (self.y_end - self.y_start) * 0.1 * (y_dir as f64);

        self.x_start += x_change;
        self.x_end += x_change;
        self.y_start += y_change;
        self.y_end += y_change;
    }

    fn zoom(&mut self, zoom_in: bool) {
        let zoom_speed = 5.0;
        let width = self.x_end - self.x_start;
        let height = self.y_end - self.y_start;
        if zoom_in {
            self.x_start += width / zoom_speed;
            self.x_end -= width / zoom_speed;
            self.y_start += height / zoom_speed;
            self.y_end -= height / zoom_speed;
        } else {
            self.x_start -= width / (zoom_speed / 2.0);
            self.x_end += width / (zoom_speed / 2.0);
            self.y_start -= height / (zoom_speed / 2.0);
            self.y_end += height / (zoom_speed / 2.0);
        }
    }

    fn update_pixel_data(&mut self) {
        let now = Instant::now();
        let width_pixels = self.window_width / self.pixel_size as i32;
        let height_pixels = self.window_height / self.pixel_size as i32;

        let fractal_space_width = self.x_end - self.x_start;
        let fractal_space_height = self.y_end - self.y_start;

        self.pixel_data
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, col)| {
                col.iter_mut().enumerate().for_each(|(y, color)| {
                    let x_f =
                        (x as f64) / (width_pixels as f64) * fractal_space_width + self.x_start;
                    let y_f = ((height_pixels as usize - y) as f64) / (height_pixels as f64)
                        * fractal_space_height
                        + self.y_start;
                    let i = match self.fractal {
                        Fractal::Mandlebrot => {
                            iterations_from_mandlebrot(x_f, y_f, self.iterations)
                        }
                        Fractal::Julia => iterations_from_julia(x_f, y_f, self.iterations),
                        Fractal::BurningShip => {
                            iterations_from_burning_ship(x_f, y_f, self.iterations)
                        }
                    };
                    if i != self.iterations {
                        *color = Some(iterations_to_color(i, self.coloring_constant));
                    } else {
                        *color = None;
                    }
                })
            });
        self.print_current_settings(now.elapsed().as_millis() as f64 / 1000.0);
    }

    fn print_current_settings(&mut self, elapsed: f64) {
        print!("{}[2J", 27 as char);
        println!("change with w,a,s,d and i,o");
        println!("\tx = {}", self.x_start);
        println!("\ty = {}", self.y_start);
        println!("\twidth = {}", self.x_end - self.x_start);
        println!("\theight = {}", self.y_end - self.y_start);
        println!("change with [,]");
        println!("\tpixel size = {}", self.pixel_size);
        println!("change with =,-");
        println!("\titerations = {}", self.iterations);
        println!("change with period,comma");
        println!("\tcoloring constant = {}", self.coloring_constant);
        println!("change with numbers");
        println!("\tfractal = {:?}", self.fractal);
        println!("time to update = {}", elapsed);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let starting_window_width: usize = 1200;
    let starting_window_height: usize = 800;
    let starting_pixel_size = 1.0;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "rust-fractals",
        [starting_window_width as u32, starting_window_height as u32],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        pixel_size: starting_pixel_size,
        x_start: -2.0,
        x_end: 1.0,
        y_start: -1.0,
        y_end: 1.0,
        iterations: 204,
        pixel_data: vec![
            vec![None; starting_window_height / starting_pixel_size as usize];
            starting_window_width / starting_pixel_size as usize
        ],
        window_width: starting_window_width as i32,
        window_height: starting_window_height as i32,
        coloring_constant: 0.05,
        fractal: Fractal::Mandlebrot,
    };

    app.update_pixel_data();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.button_args() {
            app.handle_button(&args);
        }
    }
}

fn iterations_to_color(iterations: i32, a: f32) -> [f32; 4] {
    let n = iterations as f32;
    [
        (0.5 * (a * n).sin() + 0.5),
        (0.5 * (a * n + 2.094).sin() + 0.5),
        (0.5 * (a * n + 4.188).sin() + 0.5),
        1.0,
    ]
}

fn iterations_from_mandlebrot(r0: f64, c0: f64, iterations: i32) -> i32 {
    let mut r = 0.0;
    let mut c = 0.0;
    let mut r2 = 0.0;
    let mut c2 = 0.0;
    for i in 0..iterations {
        if r2 + c2 > 4.0 {
            return i;
        }
        c = 2.0 * r * c + c0;
        r = r2 - c2 + r0;
        r2 = r * r;
        c2 = c * c;
    }
    iterations
}

fn iterations_from_julia(mut zx: f64, mut zy: f64, iterations: i32) -> i32 {
    let cx = -0.8;
    let cy = 0.156;
    for i in 0..iterations {
        if zx * zx + zy * zy > 4.0 {
            return i;
        }
        let x_temp = zx * zx - zy * zy;
        zy = 2.0 * zx * zy + cy;
        zx = x_temp + cx
    }
    iterations
}

fn iterations_from_burning_ship(x0: f64, y0: f64, iterations: i32) -> i32 {
    let y0 = -1.0 * y0;
    let mut zx = x0;
    let mut zy = y0;
    for i in 0..iterations {
        if zx * zx + zy * zy > 4.0 {
            return i;
        }
        let x_temp = zx * zx - zy * zy + x0;
        zy = (2.0 * zx * zy).abs() + y0;
        zx = x_temp;
    }
    iterations
}
