use eframe::egui;

mod filters;
mod ui;

fn main() -> eframe::Result {
    let default_width = 1200.0;
    let default_height = 700.0;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([default_width, default_height]),
        ..Default::default()
    };

    eframe::run_native(
        "Ascii app",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ui::app::AsciiApp::new(cc)))
        }),
    )
}
