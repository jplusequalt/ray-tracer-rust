use std::io::Write;

mod ray;
mod utils;
mod vec3;

use crate::utils::_utils::write_pixel;
use crate::utils::types::Color;

fn main() {
    const IMG_WIDTH: u16 = 256;
    const IMG_HEIGHT: u16 = 256;

    // render

    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..=IMG_HEIGHT - 1).rev() {
        eprint!("\rProcessing: {}", j);
        match std::io::stderr().flush() {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error),
        }
        for i in 0..IMG_WIDTH {
            let r: f32 = (i as f32) / ((IMG_WIDTH - 1) as f32);
            let g: f32 = (j as f32) / ((IMG_HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            let c = Color::from(r, g, b);

            write_pixel(&c);
        }
    }

    eprintln!("\nDone");
}
