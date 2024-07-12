use std::cmp;
use image::{DynamicImage, Pixel, RgbImage};

pub fn sobel_filter(img: &DynamicImage) -> RgbImage {
    let img_buffer = img.to_luma8();
    let (width, height) = img_buffer.dimensions();

    // Sobel operator kernels for edge detection
    let gx: [[i32; 3]; 3] = [[1, 0, -1], [2, 0, -2], [1, 0, -1]];
    let gy: [[i32; 3]; 3] = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

    let mut output_image = RgbImage::new(width, height);
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

            let grad = ((grad_x.pow(2) + grad_y.pow(2)) as f64).sqrt();
            let grad = cmp::min(255, grad as u32) as u8;

            output_image.put_pixel(x, y, image::Rgb([grad_x as u8, grad_x as u8, grad_x as u8]));
        }
    }

    output_image
}