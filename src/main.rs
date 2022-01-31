use console::Term;

fn main() {
    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => println!("Up"),
                'a' => println!("Left"),
                's' => println!("Down"),
                'd' => println!("Right"),
                _ => break
            }
        }
    }
}

// fn main() {
//    // let r = -2.0f32;
//    // let c = 0.00001f32;

//    let x_start = -2f32;
//    let x_end = 1f32;
//    let y_start = -1f32;
//    let y_end = 1f32;

//    let width = x_end - x_start;
//    let height = y_end - y_start;

//    let width_pixels = 40;
//    let height_pixels = ((width_pixels as f32) / (width / height)) as u32;
//    println!("{} {}", width_pixels, height_pixels);

//    for y in 0..height_pixels-1 {
//        for x in 0..width_pixels-1 {
//        	   let x = (x as f32) / (width_pixels as f32) * width + x_start;
// 	   let y = (y as f32) / (height_pixels as f32) * height + y_start;
// 	   if mandlebrot(x, y) {
// 	      print!("@@");
// 	   } else {
// 	     print!("  ");
// 	   }
//        }
//        println!();
//    }
// }

// fn mandlebrot(r: f32, c: f32) -> bool {
//    let r0 = r;
//    let c0 = c;
//    let mut r = 0.0;
//    let mut c = 0.0;
//    let iterations = 1000;
//    for _i in 1..iterations {
//        let square = square_complex(r, c);
//        r = square.0;
//        c = square.1;
//        r = r + r0;
//        c = c + c0;
//        if abs_complex(r, c) > 2.0 {
//        	  return false
//        }
//    }
//    true
// }

// fn square_complex(r: f32, c: f32) -> (f32, f32) {
//    let r_new = r*r - c*c;
//    let c_new = 2.*r*c;
//    (r_new, c_new)
// }

// fn abs_complex(r: f32, c: f32) -> f32 {
//    (r*r + c*c).sqrt()
// }