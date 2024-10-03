use eframe::egui;
use egui::{Align2, Color32, ColorImage, FontFamily, Id, LayerId, Order, TextStyle};
use std::fmt::Write as _;
use egui::load::SizedTexture;

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

pub fn preview_files_being_dropped(ctx: &egui::Context) {
    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
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