use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage};
use image::imageops::FilterType;
use imageproc::drawing::{draw_text_mut};
use crate::filters;

const EDGE_THRESHOLD: i32 = 6;

pub fn edge_filter(img: &DynamicImage, orig: &DynamicImage, scale_down: u32) -> (RgbImage, Vec<Vec<usize>>) {
    let reg_img = filters::sobel::sobel(&img);
    let (width, height) = reg_img.dimensions();

    // let reg_img = DynamicImage::ImageRgb8(output_image);
    reg_img.save("images/edge.png").unwrap();
    let mut ascii_img: RgbImage = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));
    let down_scaled_orig = orig.resize(width/scale_down, height/scale_down, FilterType::Nearest); // Downscale image, to sample from it

    let font = FontRef::try_from_slice(include_bytes!("../../Bescii-Mono.ttf")).unwrap();
    let scale = PxScale::from(scale_down as f32);
    
    let mut edge_vector = vec![vec![5; width as usize]; height as usize];

    // let mut down_scaled = reg_img.resize(width/scale_down, height/scale_down, FilterType::Nearest)
    // let up_scaled = down_scaled.resize(width, height, FilterType::Nearest);
    for j in (0..height).step_by(scale_down as usize) {
        for i in (0..width).step_by(scale_down as usize) {

            let mut histogram = [0, 0, 0, 0];

            for kj in j .. j + scale_down {
                if kj >= height {
                    break;
                }
                for ki in i .. i + scale_down {
                    if ki >= width {
                        break;
                    }
                    let luma_pixel = reg_img.get_pixel(ki, kj).0;
                    let char_angle = match luma_pixel {
                        [255, 255, 0] => 0,
                        [0, 255, 255] => 1,
                        [0, 255, 0] => 2,
                        [255, 0, 0] => 3,
                        _ => 4
                    };
                    if char_angle == 4 {
                        continue;
                    }
                    
                    histogram[char_angle] += 1;
                }
            }
            
            let max_char = histogram.iter().max().unwrap();
            let max_char_index = histogram.iter().position(|&x| x == *max_char).unwrap();
            if max_char < &EDGE_THRESHOLD {
                continue;
            }
            // println!("Histogram: {:?} - {} - {}", histogram, max_char_index, max_char);
            
            if max_char_index == 4 {
                continue;
            }
            
            edge_vector[j as usize][i as usize] = max_char_index;

            let char_str = match histogram.iter().enumerate().max_by_key(|x| x.1).unwrap().0 {
                0 => "/",
                1 => "\\",
                2 => "-",
                3 => "|",
                _ => " "
            };

            draw_text_mut(&mut ascii_img, down_scaled_orig.get_pixel(i/scale_down, j/scale_down).to_rgb(), i as i32, j as i32, scale, &font, char_str);
        }
    }

    // let up_scaled = down_scaled.resize(width, height, FilterType::Nearest);
    // up_scaled.save("images/downscaled.png").unwrap();

    (ascii_img, edge_vector)
}