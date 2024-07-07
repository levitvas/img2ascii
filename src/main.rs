use std::fs::File;
use std::io::Write;
use image::{GenericImageView, ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage};
use image::imageops::FilterType;
use imageproc::drawing::{draw_text_mut};
use ab_glyph::{Font, FontRef, PxScale};

fn main() {
    let name = "guts.jpg";
    let img = image::open( &format!("images/{name}")).unwrap();
    let (w, h) = img.dimensions();
    let mut ascii_img: RgbaImage = ImageBuffer::new(w, h);
    for (_, _, pixel) in ascii_img.enumerate_pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }

    let down_scaled = img.resize(w/8, h/8, FilterType::Nearest);
    let up_scaled = down_scaled.resize(w, h, FilterType::Nearest);

    let mut output = File::create(&format!("images/test-{name}")).unwrap();

    let mut luma = down_scaled.to_luma8();
    let segment_size = 26;
    let midpoints = [
        12, 38, 64, 90, 116, 142, 168, 194, 220, 245
    ];
    let chars = [" ", ".", ";", "c", "o", "P", "O", "?", "@", "■"];
    println!("{:?}", luma.dimensions());

    let mut row = 0;
    for p in luma.pixels_mut() {
        let segment_index = p.0[0] / segment_size;
        p.0[0] = midpoints[segment_index as usize];
    }
    // luma.write_to(&mut output, ImageFormat::Png).unwrap();
    let font = FontRef::try_from_slice(include_bytes!("../DejaVuSans.ttf")).unwrap();
    let height = 8.;
    let scale = PxScale::from(height);

    println!("{:?}", img.dimensions());
    for j in (0..h).step_by(8) {
        for i in (0..w).step_by(8) {
            let segment_index = luma.get_pixel(i/8, j/8).0[0] / segment_size;
            draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i/8, j/8), i as i32, j as i32, scale, &font, chars[segment_index as usize]);
            // draw_text_mut(&mut ascii_img, luma.get_pixel(i/8, j/8).to_rgba(), i as i32, j as i32, scale, &font, chars[segment_index as usize]);
        }
    }

    ascii_img.write_to(&mut output, ImageFormat::Png).unwrap();
}
