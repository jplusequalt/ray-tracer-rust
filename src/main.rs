use std::io::Write;
use std::ops;

struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn from(i: f32, j: f32, k: f32) -> Self {
        Vec3(i, j, k)
    }

    // pub fn length() -> f32 {
    //     f32::sqrt(self)
    // }

    // pub fn length_squared() -> f32 {}
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

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

            let ir = (255.999 * r) as u16;
            let ig = (255.999 * g) as u16;
            let ib = (255.999 * b) as u16;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone");
}
