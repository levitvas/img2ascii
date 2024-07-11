mod filters;

use std::fs::File;
use image::{GenericImageView, ImageFormat};

use filters::ascii::to_ascii_image;

fn main() {
    let name = "dnight.jpg";
    let scale_down = 8;
    let img = image::open( &format!("images/{name}")).unwrap();
    let ascii_img = to_ascii_image(img, scale_down);

    let mut output = File::create(&format!("images/test-{name}")).unwrap();
    ascii_img.write_to(&mut output, ImageFormat::Png).unwrap();
}
