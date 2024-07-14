use std::cmp;
use image::{DynamicImage, RgbImage};

pub fn edge_filter(img: &DynamicImage) -> RgbImage {
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

    output_image    
}