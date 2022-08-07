use crate::utils::types::Color;

pub fn write_pixel(pixel_color: &Color) {
    let ir = (255.999 * pixel_color.0) as u16;
    let ig = (255.999 * pixel_color.1) as u16;
    let ib = (255.999 * pixel_color.2) as u16;

    println!("{} {} {}", ir, ig, ib);
}
