extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::Instant;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent};
use piston::window::WindowSettings;

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
                match key {
                    Key::W | Key::Up => self.pan(0.0, 1.0),
                    Key::A | Key::Left => self.pan(-1.0, 0.0),
                    Key::S | Key::Down => self.pan(0.0, -1.0),
                    Key::D | Key::Right => self.pan(1.0, 0.0),
                    Key::I => self.zoom(true),
                    Key::O => self.zoom(false),
                    Key::Equals => self.iterations += 10,
                    Key::Minus => {
                        if self.iterations > 10 {
                            self.iterations -= 10
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
                    _ => (),
                }
                self.update_pixel_data();
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
        let scroll_speed = 10.0;
        let width = self.x_end - self.x_start;
        let height = self.y_end - self.y_start;
        if zoom_in {
            self.x_start += width / scroll_speed;
            self.x_end -= width / scroll_speed;
            self.y_start += height / scroll_speed;
            self.y_end -= height / scroll_speed;
        } else {
            self.x_start -= width / (scroll_speed / 2.0);
            self.x_end += width / (scroll_speed / 2.0);
            self.y_start -= height / (scroll_speed / 2.0);
            self.y_end += height / (scroll_speed / 2.0);
        }
    }

    fn update_pixel_data(&mut self) {
        let now = Instant::now();
        let width_pixels = self.window_width / self.pixel_size as i32;
        let height_pixels = self.window_height / self.pixel_size as i32;
        for x in 0..width_pixels {
            for y in 0..height_pixels {
                let (x_f, y_f) = self.grid_indices_to_fractal_space(
                    x,
                    height_pixels - y,
                    width_pixels,
                    height_pixels,
                );
                let i = iterations_from_mandlebrot(x_f, y_f, self.iterations);
                self.pixel_data[x as usize][y as usize] = if i == self.iterations {
                    None
                } else {
                    Some(iterations_to_color(i))
                };
            }
        }
        self.print_current_settings(now.elapsed().as_millis() as f64 / 1000.0);
    }

    fn print_current_settings(&mut self, elapsed: f64) {
        println!("------------------------------------------------------------");
        println!("x = {}", self.x_start);
        println!("y = {}", self.y_start);
        println!("width = {}", self.x_end - self.x_start);
        println!("height = {}", self.y_end - self.y_start);
        println!("pixel size = {}", self.pixel_size);
        println!("iterations = {}", self.iterations);
        println!("time to update = {}", elapsed);
    }

    fn grid_indices_to_fractal_space(&self, x: i32, y: i32, max_x: i32, max_y: i32) -> (f64, f64) {
        let fractal_space_width = self.x_end - self.x_start;
        let fractal_space_height = self.y_end - self.y_start;
        (
            (x as f64) / (max_x as f64) * fractal_space_width + self.x_start,
            (y as f64) / (max_y as f64) * fractal_space_height + self.y_start,
        )
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let starting_window_width: usize = 900;
    let starting_window_height: usize = 600;
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
        iterations: 104,
        pixel_data: vec![
            vec![None; starting_window_height / starting_pixel_size as usize];
            starting_window_width / starting_pixel_size as usize
        ],
        window_width: starting_window_width as i32,
        window_height: starting_window_height as i32,
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

fn iterations_to_color(iterations: i32) -> [f32; 4] {
    const A: f32 = 0.1;
    let n = iterations as f32;
    [
        (0.5 * (A * n).sin() + 0.5),
        (0.5 * (A * n + 2.094).sin() + 0.5),
        (0.5 * (A * n + 4.188).sin() + 0.5),
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
