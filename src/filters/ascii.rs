use std::collections::HashMap;
use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use image::imageops::FilterType;
use imageproc::drawing::{draw_text_mut};

// Converts an image to ascii image
// Requires an image dividable by 8
pub fn to_ascii_image(img: &DynamicImage, scale_down: u32, edge_vector: &Vec<Vec<usize>>, chars: &Vec<String>, upscale: u32, gamma: f32) -> RgbImage {
    let (w, h) = img.dimensions();
    let base_char_size = 8;
    let down_scaled = img.resize(w/scale_down, h/scale_down, FilterType::Nearest); // Downscale image, to sample from it
    let mut ascii_img: RgbImage = ImageBuffer::from_pixel(down_scaled.width()*base_char_size, down_scaled.height()*base_char_size, Rgb([0, 0, 0]));
    let (new_w, new_h) = ascii_img.dimensions();

    // Uncomment to save the pixelated image
    // let up_scaled = down_scaled.resize(w, h, FilterType::Nearest);
    // let mut up_output = File::create(&format!("images/pixelated-{name}")).unwrap();
    // up_scaled.write_to(&mut up_output, ImageFormat::Png).unwrap();

    let mut luma = down_scaled.to_luma32f(); // Convert to grayscale
    // let mut max_luma: f32 = 0.;
    // for x in luma.pixels() {
    //     max_luma = max_luma.max(x.0[0]);
    // };
    // 
    // let chars = [" ", ".", ";", "c", "o", "P", "O", "?", "@", "■"];
    // let char_len = chars.len() as f32;
    // 
    // for p in luma.pixels_mut() { // Quantize image to 10 segments
    //     p.0[0] = (p.0[0] * char_len).floor() / char_len;
    // }
    
    // TODO: Better quantize the high values, since now from 0.8 to 1 its the brightest
    // TODO: Allow custom pallets
    let char_len = chars.len() as f32;

    // Calculate histogram
    let mut histogram = HashMap::new();
    let mut min_luma = f32::MAX;
    let mut max_luma = f32::MIN;

    for p in luma.pixels() {
        let val = p.0[0];
        *histogram.entry((val * 255.0) as u8).or_insert(0) += 1;
        min_luma = min_luma.min(val);
        max_luma = max_luma.max(val);
    }

    // Calculate cumulative distribution
    let total_pixels = (luma.width() * luma.height()) as f32;
    let mut cumsum = 0.0;
    let mut equalization_map = HashMap::new();

    for i in 0..=255 {
        if let Some(&count) = histogram.get(&(i as u8)) {
            cumsum += count as f32 / total_pixels;
            equalization_map.insert(i as u8, cumsum);
        }
    }

    // Quantize with equalization and gamma correction
    for p in luma.pixels_mut() {
        let normalized = (p.0[0] - min_luma) / (max_luma - min_luma);
        let equalized = equalization_map.get(&((normalized * 255.0) as u8))
            .unwrap_or(&normalized);

        // Apply gamma correction
        let gamma_corrected = equalized.powf(1.0 / gamma);

        // Quantize to character levels
        p.0[0] = (gamma_corrected * (char_len - 1.0)).round() / (char_len - 1.0);
    }

    let font = FontRef::try_from_slice(include_bytes!("../../Bescii-Mono.ttf")).unwrap();
    let scale = PxScale::from(base_char_size as f32);

    for j in 0..down_scaled.height() {
        for i in 0..down_scaled.width() {

            let luma_pixel = luma.get_pixel(i, j).0[0];
            // let rgb_pixel = Rgb([(luma_pixel * 255.) as u8, (luma_pixel * 255.) as u8, (luma_pixel * 255.) as u8]);
            
            if edge_vector[(j * scale_down) as usize][(i * scale_down) as usize] != 5 {
                let char_str = match edge_vector[(j * scale_down) as usize][(i * scale_down) as usize] {
                    0 => "/",
                    1 => "\\",
                    2 => "-",
                    3 => "|",
                    _ => " "
                };

                draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i, j).to_rgb(), (i * base_char_size) as i32, (j * base_char_size) as i32, scale, &font, char_str);
                // draw_text_mut(&mut ascii_img, rgb_pixel, i as i32, j as i32, scale, &font, char_str); // for mono color   
                
            } else {
                let mut index = (luma_pixel * char_len) as usize;
                if index >= chars.len() {
                    index = chars.len() - 1;
                }
                draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i, j).to_rgb(), (i * base_char_size) as i32, (j * base_char_size) as i32, scale, &font, &*chars[index]);

                // Uncomment to use monochrome color
                // draw_text_mut(&mut ascii_img, rgb_pixel, i as i32, j as i32, scale, &font, chars[index]); // for mono color   
            }
        }
    }

    if upscale > 1 {
        DynamicImage::ImageRgb8(ascii_img).resize(new_w * upscale, new_h * upscale, FilterType::Nearest).to_rgb8()
    } else {
        ascii_img
    }
}