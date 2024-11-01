use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage};
use image::imageops::FilterType;
use imageproc::drawing::{draw_text_mut};
use crate::filters;

pub fn edge_filter(img: &DynamicImage, orig: &DynamicImage, scale_down: u32, edge_threshold: i32) -> (RgbImage, Vec<Vec<usize>>) {
    let reg_img = filters::sobel::sobel(&img);
    let (width, height) = reg_img.dimensions();

    reg_img.save("images/edge.png").unwrap();
    let mut ascii_img: RgbImage = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));
    let down_scaled_orig = orig.resize(width/scale_down, height/scale_down, FilterType::Nearest);

    let font = FontRef::try_from_slice(include_bytes!("../../Bescii-Mono.ttf")).unwrap();
    let scale = PxScale::from(scale_down as f32);

    let mut edge_vector = vec![vec![5; width as usize]; height as usize];
    
    for j in (0..height).step_by(scale_down as usize) {
        for i in (0..width).step_by(scale_down as usize) {
            let mut histogram = [0, 0, 0, 0];

            for kj in j..std::cmp::min(j + scale_down, height) {
                for ki in i..std::cmp::min(i + scale_down, width) {
                    let luma_pixel = reg_img.get_pixel(ki, kj).0;
                    let char_angle = match luma_pixel {
                        [255, 255, 0] => 0,
                        [0, 255, 255] => 1,
                        [0, 255, 0] => 2,
                        [255, 0, 0] => 3,
                        _ => 4
                    };
                    if char_angle != 4 {
                        histogram[char_angle] += 1;
                    }
                }
            }

            if let Some((max_char_index, &max_char)) = histogram.iter().enumerate().max_by_key(|&(_, count)| count) {
                if max_char >= edge_threshold {
                    edge_vector[j as usize][i as usize] = max_char_index;

                    let char_str = match max_char_index {
                        0 => "/",
                        1 => "\\",
                        2 => "-",
                        3 => "|",
                        _ => " "
                    };

                    // Use the original image color for drawing
                    let color = down_scaled_orig.get_pixel(i/scale_down, j/scale_down).to_rgb();
                    draw_text_mut(&mut ascii_img, color, i as i32, j as i32, scale, &font, char_str);
                }
            }
        }
    }
    
    (ascii_img, edge_vector)
}