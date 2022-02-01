use console::Term;

fn main() {
    let stdout = Term::buffered_stdout();

    let mut x_start = -2f64;
    let mut x_end = 1f64;
    let mut y_start = -1f64;
    let mut y_end = 1f64;
    let mut iterations = 101;
    let mut width_pixels = 100;

    print_mandlebrot(x_start, x_end, y_start, y_end, iterations, width_pixels);

    loop {
        if let Ok(character) = stdout.read_char() {
            print!("{}[2J", 27 as char);
            match character {
                'w' => {
                    let change = (x_end - x_start) / 10.0;
                    y_start -= change;
                    y_end -= change;
                }
                'a' => {
                    let change = (y_end - y_start) / 10.0;
                    x_start -= change;
                    x_end -= change;
                }
                's' => {
                    let change = (x_end - x_start) / 10.0;
                    y_start += change;
                    y_end += change;
                }
                'd' => {
                    let change = (y_end - y_start) / 10.0;
                    x_start += change;
                    x_end += change;
                }
                'i' => {
                    let width = x_end - x_start;
                    let height = y_end - y_start;
                    x_start += width / 4.0;
                    x_end -= width / 4.0;
                    y_start += height / 4.0;
                    y_end -= height / 4.0;
                }
                'o' => {
                    let width = x_end - x_start;
                    let height = y_end - y_start;
                    x_start -= width / 2.0;
                    x_end += width / 2.0;
                    y_start -= height / 2.0;
                    y_end += height / 2.0;
                }
                '=' => {
                    iterations += 10;
                    println!("new iterations: {}", iterations);
                }
                '-' => {
                    if iterations > 10 {
                        iterations -= 10;
                    }
                    println!("new iterations: {}", iterations);
                }
                '.' => {
                    width_pixels += 5;
                }
                ',' => {
                    if width_pixels > 5 {
                        width_pixels -= 5;
                    }
                }
                x => {
                    println!("unrecognized key: {}", x);
                    continue;
                }
            }
            print_mandlebrot(x_start, x_end, y_start, y_end, iterations, width_pixels);
        }
    }
}

fn print_mandlebrot(
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    iterations: u32,
    width_pixels: u32,
) {
    let width = x_end - x_start;
    let height = y_end - y_start;
    let height_pixels = ((width_pixels as f64) * (height / width) / 2.0) as u32;
    let width_pixel_distance = width / (width_pixels as f64);
    let height_pixel_distance = height / (height_pixels as f64);

    println!("{}", "_".repeat(width_pixels as usize));

    for y in 0..height_pixels {
        print!("|");
        for x in 0..width_pixels {
            let x = (x as f64) * width_pixel_distance + x_start;
            let y = (y as f64) * height_pixel_distance + y_start;
            let i = in_mandlebrot(x, y, iterations);
            let (r, g, b) = if i != iterations {
                (
                    ((i as f32) * (255 as f32 / iterations as f32)) as u32,
                    ((i as f32) * (255 as f32 / iterations as f32)) as u32,
                    140,
                )
            } else {
                (0, 0, 0)
            };
            print!("\x1b[48;2;{};{};{}m \x1b[0m", r, g, b);
        }
        println!("|");
    }
    println!("{}", "_".repeat(width_pixels as usize));
}

fn in_mandlebrot(r0: f64, c0: f64, iterations: u32) -> u32 {
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
