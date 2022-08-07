use crate::utils::types::Color;

pub fn write_pixel(pixel_color: &Color) {
    let ir = (255.999 * pixel_color.x) as u16;
    let ig = (255.999 * pixel_color.y) as u16;
    let ib = (255.999 * pixel_color.z) as u16;

    println!("{} {} {}", ir, ig, ib);
}
