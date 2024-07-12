mod filters;

use std::fs::File;
use image::{DynamicImage, GenericImageView, ImageFormat};
use imageproc::drawing::Canvas;
use show_image::{create_window, event, ImageInfo, ImageView};
use filters::ascii::to_ascii_image;
use filters::sobel::sobel_filter;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let name = "circle.png";
    let scale_down = 8;
    let img = image::open( &format!("images/{name}")).expect("Failed to open image");
    
    // let ascii_img = to_ascii_image(img, scale_down);
    let sobel = sobel_filter(&img);
    sobel.save(&format!("images/sobel-{name}")).unwrap();
    // let mut output = File::create(&format!("images/test-{name}")).unwrap();
    // ascii_img.write_to(&mut output, ImageFormat::Png).unwrap();
    let dynamic_image = DynamicImage::ImageRgba8(sobel);
    let image_data = dynamic_image.to_rgb8();

    
    // Create an `ImageView` from the `DynamicImage`
    let image_view = ImageView::new(ImageInfo::rgb8(dynamic_image.width(), dynamic_image.height()), &image_data);

    // Create a window and display the image
    let window = create_window("Sobel Filtered Image", Default::default())?;
    window.set_image("sobel-filtered-image", image_view)?;

    for event in window.event_channel().map_err(|e| e.to_string())? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if !event.is_synthetic && event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                println!("Escape pressed!");
                break;
            }
        }
    }

    Ok(())
}
