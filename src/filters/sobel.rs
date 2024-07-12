use std::cmp;
use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, RgbaImage, RgbImage};
use image::imageops::FilterType;
use imageproc::drawing::{draw_text, draw_text_mut};

const EDGE_THRESHOLD: i32 = 6;

pub fn sobel_filter(img: &DynamicImage) -> RgbaImage {
    let img_buffer = img.to_luma8();
    let (width, height) = img_buffer.dimensions();

    // Sobel operator kernels for edge detection
    let gx: [[i32; 3]; 3] = [[1, 0, -1], [2, 0, -2], [1, 0, -1]];
    let gy: [[i32; 3]; 3] = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

    let mut output_image = RgbImage::new(width, height);

    let mut max_grad_x = 0;
    let mut max_grad_y = 0;
    let mut max_grad = 0;

    let mut grad_x_vec = vec![vec![0; width as usize]; height as usize];
    let mut grad_y_vec = vec![vec![0; width as usize]; height as usize];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut grad_x = 0;
            let mut grad_y = 0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let pixel = img_buffer.get_pixel(x + kx - 1, y + ky - 1).0[0] as i32;
                    grad_x += gx[ky as usize][kx as usize] * pixel;
                    grad_y += gy[ky as usize][kx as usize] * pixel;
                }
            }

            grad_x_vec[y as usize][x as usize] = grad_x;
            grad_y_vec[y as usize][x as usize] = grad_y;
            max_grad_x = cmp::max(max_grad_x, grad_x as i32);
            max_grad_y = cmp::max(max_grad_y, grad_y as i32);

            let grad = ((grad_x.pow(2) + grad_y.pow(2)) as f64).sqrt();
            max_grad = cmp::max(max_grad, grad as i32);
        }
    }

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let grad_x = grad_x_vec[y as usize][x as usize];
            let grad_y = grad_y_vec[y as usize][x as usize];

            let grad = ((grad_x.pow(2) + grad_y.pow(2)) as f64).sqrt();
            // let grad = cmp::min(255, grad as u32) as u8;

            let angle = f32::atan2(grad_y as f32, grad_x as f32);

            let normalized_angle = ((angle * 0.5) / std::f32::consts::PI) + 0.5;
            let quantized = (normalized_angle * 4.).floor() / 4.;


            if grad != 0.{
                let normed = (grad as f32 * 255. / max_grad as f32) as u8;
                // println!("Angle: {}, Normalized: {}", angle, normalized_angle);
                if (0. < normalized_angle && normalized_angle < 0.25) || (0.5 < normalized_angle && normalized_angle < 0.75) {
                    output_image.put_pixel(x, y, image::Rgb([255, 255, 0]));
                } else if (0.25 < normalized_angle && normalized_angle < 0.5) || (0.75 < normalized_angle && normalized_angle < 1.) {
                    output_image.put_pixel(x, y, image::Rgb([0, 255, 255]));
                } else if 0.25 == normalized_angle || 0.75 == normalized_angle {
                    output_image.put_pixel(x, y, image::Rgb([0, 255, 0]));
                }
                else if 0.5 == normalized_angle || 1. == normalized_angle {
                    output_image.put_pixel(x, y, image::Rgb([255, 0, 0]));
                }
                // output_image.put_pixel(x, y, image::Rgb([normed, normed, normed]));
            }
        }
    }

    println!("Max grad: {}", max_grad_x);
    println!("Max grad: {}", max_grad_y);

    let reg_img = DynamicImage::ImageRgb8(output_image);
    let mut ascii_img: RgbaImage = ImageBuffer::from_pixel(width, height, image::Rgba([0, 0, 0, 255]));
    let scale_down = 8;

    let font = FontRef::try_from_slice(include_bytes!("../../Bescii-Mono.ttf")).unwrap();
    let h = scale_down as f32;
    let scale = PxScale::from(h);

    let mut down_scaled = reg_img.resize(width/scale_down, height/scale_down, FilterType::Nearest); // Downscale image, to sample from it
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
                        [255, 255, 0, 255] => 0,
                        [0, 255, 255, 255] => 1,
                        [0, 255, 0, 255] => 2,
                        [255, 0, 0, 255] => 3,
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
            println!("Histogram: {:?} - {} - {}", histogram, max_char_index, max_char);

            let char_str = match histogram.iter().enumerate().max_by_key(|x| x.1).unwrap().0 {
                0 => "/",
                1 => "\\",
                2 => "-",
                3 => "|",
                _ => " "
            };

            let color = match char_str {
                "/" => [255, 255, 0, 255],
                "\\" => [0, 255, 255, 255],
                "-" => [0, 255, 0, 255],
                "|" => [255, 0, 0, 255],
                _ => [0, 0, 0, 255]
            };
            
            down_scaled.put_pixel(i/scale_down, j/scale_down, image::Rgba(color));

            draw_text_mut(&mut ascii_img, down_scaled.get_pixel(i/scale_down, j/scale_down), i as i32, j as i32, scale, &font, char_str);
        }
    }

    let up_scaled = down_scaled.resize(width, height, FilterType::Nearest);
    up_scaled.save("images/downscaled.png").unwrap();

    println!("Finised");

    ascii_img
}