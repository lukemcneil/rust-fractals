extern crate nannou;
use nannou::prelude::*;

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 400;

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .run();
}

struct Model {
    pixel_size: i32,
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    iterations: i32,
}

fn model(_app: &App) -> Model {
    Model {
        pixel_size: 4,
        x_start: -2.0,
        x_end: 1.0,
        y_start: -1.0,
        y_end: 1.0,
        iterations: 101,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { id: _, simple } = event {
        if let Some(window_event) = simple {
            match window_event {
                WindowEvent::MouseMoved(_point) => {
                    // model.x = point[0];
                    // model.y = point[1];
                    println!("{:?}", window_event)
                }
                // WindowEvent::MouseWheel(mouse_scroll_delta, touch_phase) => {
                //     if let MouseScrollDelta::LineDelta(hor, ver) = mouse_scroll_delta {}
                // }
                WindowEvent::KeyPressed(key) => match key {
                    Key::Up | Key::W => {
                        let change = (model.y_end - model.y_start) * 0.1;
                        model.y_start += change;
                        model.y_end += change;
                    }
                    Key::Down | Key::S => {
                        let change = (model.y_end - model.y_start) * 0.1;
                        model.y_start -= change;
                        model.y_end -= change;
                    }
                    Key::Right | Key::D => {
                        let change = (model.x_end - model.x_start) * 0.1;
                        model.x_start += change;
                        model.x_end += change;
                    }
                    Key::Left | Key::A => {
                        let change = (model.x_end - model.x_start) * 0.1;
                        model.x_start -= change;
                        model.x_end -= change;
                    }
                    Key::I => {
                        let width = model.x_end - model.x_start;
                        let height = model.y_end - model.y_start;
                        model.x_start += width / 4.0;
                        model.x_end -= width / 4.0;
                        model.y_start += height / 4.0;
                        model.y_end -= height / 4.0;
                    }
                    Key::O => {
                        let width = model.x_end - model.x_start;
                        let height = model.y_end - model.y_start;
                        model.x_start -= width / 2.0;
                        model.x_end += width / 2.0;
                        model.y_start -= height / 2.0;
                        model.y_end += height / 2.0;
                    }
                    Key::K => {
                        model.pixel_size = std::cmp::max(1, model.pixel_size - 1);
                    }
                    Key::L => {
                        model.pixel_size = std::cmp::min(WINDOW_HEIGHT / 2, model.pixel_size + 1);
                    }
                    Key::Comma => {
                        model.iterations += 50;
                    }
                    Key::Period => {
                        model.iterations = std::cmp::max(10, model.iterations - 50);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    println!("{}", app.fps());
    let draw = app.draw();
    draw.background().color(PLUM);
    draw_mandlebrot(&model, &draw, &frame);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_mandlebrot(model: &Model, draw: &Draw, frame: &Frame) {
    let width_pixels = frame.texture_size()[0] as i32 / model.pixel_size;
    let height_pixels = frame.texture_size()[1] as i32 / model.pixel_size;
    let width_pixel_distance = (model.x_end - model.x_start) / (width_pixels as f64);
    let height_pixel_distance = (model.y_end - model.y_start) / (height_pixels as f64);
    for y in 0..height_pixels {
        for x in 0..width_pixels {
            let x_mandlebrot_space = (x as f64) * width_pixel_distance + model.x_start;
            let y_mandlebrot_space = (y as f64) * height_pixel_distance + model.y_start;
            let i = iterations_from_mandlebrot(
                x_mandlebrot_space,
                y_mandlebrot_space,
                model.iterations,
            );
            if i == model.iterations {
                draw.rect()
                    .color(BLACK)
                    .x((x * model.pixel_size) as f32 - (frame.texture_size()[0] / 2) as f32)
                    .y((y * model.pixel_size) as f32 - (frame.texture_size()[1] / 2) as f32)
                    .w(model.pixel_size as f32)
                    .h(model.pixel_size as f32);
            }
        }
    }
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
