use std::cmp::max;
use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use image::imageops::FilterType;
use imageproc::drawing::draw_text_mut;

// Converts an image to ascii image
// Requires an image dividable by 8
pub fn to_ascii_image(img: &DynamicImage, scale_down: u32, edge_vector: &Vec<Vec<usize>>) -> RgbImage {
    let (w, h) = img.dimensions();
    let mut ascii_img: RgbImage = ImageBuffer::from_pixel(w, h, Rgb([0, 0, 0]));
    let down_scaled = img.resize(w/scale_down, h/scale_down, FilterType::Nearest); // Downscale image, to sample from it

    // Uncomment to save the pixelated image
    // let up_scaled = down_scaled.resize(w, h, FilterType::Nearest);
    // let mut up_output = File::create(&format!("images/pixelated-{name}")).unwrap();
    // up_scaled.write_to(&mut up_output, ImageFormat::Png).unwrap();

    let mut luma = down_scaled.to_luma32f(); // Convert to grayscale
    let mut max_luma: f32 = 0.;
    for x in luma.pixels() {
        max_luma = max_luma.max(x.0[0]);
    };
    
    let chars = [" ", ".", ";", "c", "o", "P", "O", "?", "@", "■"];
    let char_len = chars.len() as f32;

    for p in luma.pixels_mut() { // Quantize image to 10 segments
        p.0[0] = (p.0[0] * char_len).floor() / char_len;
    }

    let font = FontRef::try_from_slice(include_bytes!("../../Bescii-Mono.ttf")).unwrap();
    let height = scale_down as f32;
    let scale = PxScale::from(height);

    for j in (0..h).step_by(scale_down as usize) {
        for i in (0..w).step_by(scale_down as usize) {

            let luma_pixel = luma.get_pixel(i/scale_down, j/scale_down).0[0];
            let rgb_pixel = Rgb([(luma_pixel * 255.) as u8, (luma_pixel * 255.) as u8, (luma_pixel * 255.) as u8]);
            
            if edge_vector[j as usize][i as usize] != 5 {
                let char_str = match edge_vector[j as usize][i as usize] {
                    0 => "/",
                    1 => "\\",
                    2 => "-",
                    3 => "|",
                    _ => " "
                };

                draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i/scale_down, j/scale_down).to_rgb(), i as i32, j as i32, scale, &font, char_str);
                // draw_text_mut(&mut ascii_img, rgb_pixel, i as i32, j as i32, scale, &font, char_str); // for mono color   
                
            } else {
                let mut index = (luma_pixel * char_len) as usize;
                if index >= chars.len() {
                    index = 9;
                }
                draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i/scale_down, j/scale_down).to_rgb(), i as i32, j as i32, scale, &font, chars[index]);

                // Uncomment to use monochrome color
                // draw_text_mut(&mut ascii_img, rgb_pixel, i as i32, j as i32, scale, &font, chars[index]); // for mono color   
            }
        }
    }
    
    ascii_img
}