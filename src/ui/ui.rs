use std::ptr::read;
use eframe::egui;
use egui::{FontId, RichText, Vec2};
use std::time::Duration;
use image::imageops::FilterType;
use image::ImageReader;
use imageproc::drawing::Canvas;
use crate::ui;

pub fn top_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Image to Ascii").font(FontId::proportional(18.0)),
                );
                ui.add_space(6.0);
            });
        });
    });
}

pub fn bottom_panel(ctx: &egui::Context, app: &mut ui::app::AsciiApp) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .min_height(ctx.screen_rect().height() / 5.)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Image Control");
                ui.add_space(15.);
                ui.columns(6, |cols| {
                    cols[1].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Change Image").min_size(Vec2::from([
                                ui.available_width() * 0.9,
                                ui.available_height() * 0.5,
                            ])))
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
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
                                    app.toasts.info(format!("Image width will be cropped by {}", crop_pixels)).set_duration(Option::from(Duration::from_secs(5)));
                                    app.orig_img = app.orig_img.crop(0, 0, app.orig_img.width()-crop_pixels, app.orig_img.height());
                                }
                                if app.orig_img.height() % (app.scale_factors[app.scale_factors.len()-1] as u32) != 0 {
                                    let crop_pixels = app.orig_img.height() % (app.scale_factors[app.scale_factors.len()-1] as u32);
                                    app.toasts.info(format!("Image height will be cropped by {}", crop_pixels)).set_duration(Option::from(Duration::from_secs(5)));
                                    app.orig_img = app.orig_img.crop(0, 0, app.orig_img.width(), app.orig_img.height()-crop_pixels);
                                }
                                app.changed = true;
                                app.update_images(app.image_type as u32);
                            }
                        }
                    });
                    cols[4].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Save Image").min_size(Vec2::from([
                                ui.available_width() * 0.9,
                                ui.available_height() * 0.5,
                            ])))
                            .clicked()
                        {
                            let res = rfd::FileDialog::new()
                                .set_file_name(&format!("ascii-{}", app.picked_path.as_ref().unwrap().file_name().unwrap().to_str().unwrap()))
                                .add_filter("PNG", &["png"])
                                .save_file();

                            println!("The user choose: {:#?}", res);
                            if let Some(path) = res {
                                match app.image_type { 
                                    1 => {app.ascii_img.as_ref().unwrap().save(path).unwrap();},
                                    2 => {app.sobel_img.as_ref().unwrap().save(path).unwrap();},
                                    3 => {app.gaus_img.as_ref().unwrap().save(path).unwrap();},
                                    _ => {app.toasts.info("Did not save!").set_duration(Option::from(Duration::from_secs(5)));},
                                }
                                app.toasts.success("Saved!").set_duration(Option::from(Duration::from_secs(5)));
                            } else {
                                app.toasts.error("Could not save the image").set_duration(Option::from(Duration::from_secs(5)));
                            }
                        }
                    });
                });
            });
        });
}

pub fn central_panel(ctx: &egui::Context, app: &mut ui::app::AsciiApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.allocate_ui_with_layout(
                [2. * ui.available_width() / 3., ui.available_height()].into(),
                egui::Layout::top_down(egui::Align::TOP),
                |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui
                            .add_sized(
                                Vec2::new(ui.available_width() * 0.9, ui.available_height() * 0.9),
                                egui::Image::new(ui::utils::convert_image_to_texture(
                                    match app.image_type {
                                        0 => {&app.orig_img},
                                        1 => {&app.ascii_img.as_ref().unwrap()},
                                        2 => {&app.sobel_img.as_ref().unwrap()},
                                        3 => {&app.gaus_img.as_ref().unwrap()},
                                        _ => {panic!("Invalid image type")},
                                    },
                                    ui,
                                ).unwrap()).shrink_to_fit(),
                            )
                    });
                    ui.add_space(ui.available_height() * 0.2);
                    let buttons = ["Original", "Ascii", "Sobel", "Gaussian"];
                    ui.columns(6, |cols| {
                        for (i, &button) in buttons.iter().enumerate() {
                            cols[i + 1].vertical_centered(|ui| {
                                let is_selected = i == app.image_type;
                                let button = egui::Button::new(button)
                                    .min_size(Vec2::from([
                                        ui.available_width() * 0.9,
                                        ui.available_height() * 0.9,
                                    ]))
                                    .fill(if is_selected {
                                        egui::Color32::from_rgb(100, 100, 100)
                                    } else {
                                        egui::Color32::from_rgb(60, 60, 60)
                                    });
                                
                                if ui.add(button).clicked() {
                                    if app.image_type != i {
                                        app.image_type = i;
                                        app.update_images(i as u32);
                                    }
                                }
                            });
                        }
                    });
                },
            );

            ui.separator();

            ui.allocate_ui_with_layout(
                [ui.available_width(), ui.available_height()].into(),
                egui::Layout::top_down(egui::Align::TOP),
                |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.heading("Parameter Control");
                        ui.add_space(25.0);
                        ui.add(egui::Slider::new(&mut app.sigma_one, 1..=20).text("Sigma One"));
                        ui.add_space(20.0);
                        ui.add(egui::Slider::new(&mut app.sigma_two, 1..=50).text("Sigma Two"));
                        ui.add_space(20.0);
                        ui.add(egui::Slider::new(&mut app.threshold, 1..=50).text("Threshold"));
                        ui.add_space(20.0);
                        ui.add(egui::Slider::new(&mut app.edge_threshold, 1..=10).text("Edge Threshold")).on_hover_text("Determines how many pixels are needed to form an edge");
                        ui.add_space(20.0);
                        let _tau_slider = ui.add(egui::Slider::new(&mut app.tau, 0.1..=1.0).text("Tau"));

                        ui.add_space(40.0);
                        ui.add(egui::Slider::new(&mut app.scale_factor_id, 0..=(app.scale_factors.len() as i32)-1).text("Scale Down").custom_formatter(|x, _| {
                            format!("{}", app.scale_factors[x as usize])
                        }));
                        ui.add_space(20.0);
                        ui.add(egui::Slider::new(&mut app.up_scale_factor_id, 0..=(app.up_scale_factors.len() as i32)-1).text("Upscale").custom_formatter(|x, _| {
                            format!("{}", app.up_scale_factors[x as usize])
                        }));
                        
                        ui.add_space(ui.available_height()*0.8);
                        if ui.add(egui::Button::new("Apply")
                            .min_size(Vec2::from([ui.available_width(), ui.available_height()*0.9]))
                            .fill(egui::Color32::from_rgb(60, 60, 60))
                            ).on_hover_text("Apply the changes to the image").clicked() {
                                app.changed = true;
                                app.update_images(app.image_type as u32);
                                app.toasts.success("Applied!").set_duration(Option::from(Duration::from_secs(2)));
                            };
                    });
                },
            );
        });
    });
}