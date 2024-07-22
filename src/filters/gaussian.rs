use image::{DynamicImage, RgbImage};
use imageproc::filter::gaussian_blur_f32;

pub fn gaussian_diff(img: &DynamicImage, sigma_one: f32, sigma_two: f32, threshold: i32, tau: f32) -> RgbImage {
    let grayscale = img.to_luma8();
    grayscale.save("images/grayscale.png").unwrap();
    let first_gaus = gaussian_blur_f32(&grayscale, sigma_one);
    let second_gaus = gaussian_blur_f32(&grayscale, sigma_two);
    
    let mut difference = RgbImage::new(grayscale.width(), grayscale.height());
    for y in 0..grayscale.height() {
        for x in 0..grayscale.width() {
            let diff = first_gaus.get_pixel(x, y).0[0] as i32 - (tau * second_gaus.get_pixel(x, y).0[0] as f32) as i32;
            let diff = if diff < threshold { 0 } else { 255 };
            difference.put_pixel(x, y, image::Rgb([diff as u8, diff as u8, diff as u8]));
        }
    }
    difference.save("images/difference.png").unwrap();
    
    difference
}