use eframe::egui;
use egui::{Align2, Color32, ColorImage, FontFamily, Id, LayerId, Order, TextStyle};
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Duration;
use egui::load::SizedTexture;
use image::ImageReader;
use crate::ui::app::AsciiApp;

pub fn configure_font(ctx: &egui::Context) {
    ctx.set_pixels_per_point(1.0);

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Bescii".to_owned(),
        egui::FontData::from_static(include_bytes!("../../Bescii-Mono.ttf")),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Bescii".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("Bescii".to_owned());

    ctx.set_fonts(fonts);
}

pub fn change_image(path: PathBuf, app: &mut AsciiApp) -> bool {
    app.picked_path = Some(path);
    app.orig_img = match ImageReader::open(app.picked_path.clone().unwrap()) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => panic!("Failed to decode image: {:?}", e),
            },
            Err(e) => panic!("Failed to guess image format: {:?}", e),
        },
        Err(e) => panic!("Failed to open image: {:?}", e),
    };

    // Crops the image, so that division works well without errors
    if app.orig_img.width() % (app.scale_factors[app.scale_factors.len()-1] as u32) != 0 {
        let crop_pixels = app.orig_img.width() % (app.scale_factors[app.scale_factors.len()-1] as u32);
        app.toasts.info(format!("Image width will be cropped by {}", crop_pixels)).set_duration(Option::from(Duration::from_secs(3)));
        app.orig_img = app.orig_img.crop(0, 0, app.orig_img.width()-crop_pixels, app.orig_img.height());
    }
    if app.orig_img.height() % (app.scale_factors[app.scale_factors.len()-1] as u32) != 0 {
        let crop_pixels = app.orig_img.height() % (app.scale_factors[app.scale_factors.len()-1] as u32);
        app.toasts.info(format!("Image height will be cropped by {}", crop_pixels)).set_duration(Option::from(Duration::from_secs(3)));
        app.orig_img = app.orig_img.crop(0, 0, app.orig_img.width(), app.orig_img.height()-crop_pixels);
    }
    app.changed = true;
    app.update_images(app.image_type as u32);
    true
}

pub fn save_image(app: &mut AsciiApp) -> bool {
    let res = rfd::FileDialog::new()
        .set_file_name(&format!("ascii-{}", app.picked_path.as_ref().unwrap().file_name().unwrap().to_str().unwrap()))
        .add_filter("PNG", &["png"])
        .save_file();

    if let Some(path) = res {
        match app.image_type {
            1 => {app.ascii_img.as_ref().unwrap().save(path).unwrap();},
            2 => {app.sobel_img.as_ref().unwrap().save(path).unwrap();},
            3 => {app.gaus_img.as_ref().unwrap().save(path).unwrap();},
            _ => {app.toasts.info("Did not save!").set_duration(Option::from(Duration::from_secs(5)));},
        }
        true
    } else {
        false
    }
}

// TODO: Implement this
pub fn preview_files_being_dropped(ctx: &egui::Context, app: &mut AsciiApp) {
    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping file:\n".to_owned();
            if i.raw.hovered_files.len() == 1 {
                if let Some(path) = &i.raw.hovered_files[0].path {
                    write!(text, "{}", path.display()).ok();
                } else if !i.raw.hovered_files[0].mime.is_empty() {
                    write!(text, "{}", i.raw.hovered_files[0].mime).ok();
                } else {
                    text += "???";
                }
            } else {
                text = "More than one file!".to_owned();
            }
            text
        });


        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
    if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
        println!("Dropped files: {:#?}", ctx.input(|i| i.raw.dropped_files.clone()));
        if ctx.input(|i| {
            if i.raw.dropped_files.len() == 1 {
                let res = change_image(i.raw.dropped_files[0].path.clone().unwrap(), app);
                res
            } else {
                false
            }
        }) {
            app.toasts.success("Changed image!").set_duration(Option::from(Duration::from_secs(1)));
        } else {
            app.toasts.error("Failed to change image!").set_duration(Option::from(Duration::from_secs(1)));
        }
    }
}

pub fn convert_image_to_texture(img: &image::DynamicImage, ui: &egui::Ui) -> Option<SizedTexture> {
    let size = [img.width() as _, img.height() as _];
    let image_buffer = img.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let texture_handle =  ui.ctx().load_texture(
        "my-image",
        color_image,
        Default::default()
    );

    Some(SizedTexture::from_handle(&texture_handle))
}