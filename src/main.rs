mod filters;

use std::fs::File;
use image::{DynamicImage, GenericImageView, ImageFormat};
use imageproc::drawing::Canvas;
use imageproc::filter::gaussian_blur_f32;
use show_image::{create_window, event, ImageInfo, ImageView};
use filters::ascii::to_ascii_image;
use filters::edge_detect::edge_filter;
use filters::gaussian::gaussian_diff;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let name = "dnight.jpg";
    let scale_down = 8;
    let img = image::open( &format!("images/{name}")).expect("Failed to open image");
    
    let gaus = gaussian_diff(&img, 7., 20., 10, 0.9);
    let gaus_dyn = DynamicImage::ImageRgb8(gaus.clone());
    let (sobel, edges) = edge_filter(&gaus_dyn, &img);
    let ascii_img = to_ascii_image(&img, scale_down, &edges);

    sobel.save(&format!("images/sobel-{name}")).unwrap();
    gaus.save(&format!("images/gaus-{name}")).unwrap();
    ascii_img.save(&format!("images/ascii-{name}")).unwrap();

    let dynamic_image = DynamicImage::ImageRgb8(ascii_img);
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
