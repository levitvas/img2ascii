mod filters;

use std::fs::File;
use image::{GenericImageView, ImageFormat};

use filters::ascii::to_ascii_image;
use filters::sobel::sobel_filter;

fn main() {
    let name = "Bikesgray.jpg";
    let scale_down = 8;
    let img = image::open( &format!("images/{name}")).expect("Failed to open image");
    
    // let ascii_img = to_ascii_image(img, scale_down);
    let sobel = sobel_filter(&img);
    sobel.save(&format!("images/sobel-{name}")).unwrap();
    // let mut output = File::create(&format!("images/test-{name}")).unwrap();
    // ascii_img.write_to(&mut output, ImageFormat::Png).unwrap();
}
