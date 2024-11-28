use eframe::egui;
use egui::{popup_below_widget, FontId, Id, PopupCloseBehavior, RichText, Vec2};
use std::time::Duration;
use eframe::emath::TSTransform;
use image::ImageReader;
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
                ui.columns(7, |cols| {
                    cols[1].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Change Image").min_size(Vec2::from([
                                ui.available_width() * 0.95,
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
                    cols[3].vertical_centered(|ui| {
                        let response = ui.add(egui::Button::new("Change asciis").min_size(Vec2::from([
                            ui.available_width() * 0.95,
                            ui.available_height() * 0.5,
                        ])));
                        let popup_id = Id::new("popup_id");
                        
                        if response.clicked() {
                            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        }

                        let below = egui::AboveOrBelow::Above;
                        let close_on_click_outside = egui::popup::PopupCloseBehavior::IgnoreClicks;
                        egui::popup::popup_above_or_below_widget(ui, popup_id, &response, below, close_on_click_outside, |ui| {
                            ui.set_min_width(app.charset.len() as f32 * 37.0); 
                            ui.set_min_height(150.0);
                            ui.add_space(10.0);
                            ui.label("Chars separated by comma");
                            ui.add_space(20.0);
                            if ui.add(egui::TextEdit::singleline(&mut app.charset_text).hint_text("Write something here")).lost_focus() {
                                if app.check_charset_correctness() {
                                    app.charset_text = app.charset.join(", ");
                                    app.toasts.success("Changed charset!").set_duration(Option::from(Duration::from_secs(1)));
                                } else {
                                    app.toasts.error("Invalid charset!").set_duration(Option::from(Duration::from_secs(3)));
                                }
                            };
                        });
                    });
                    cols[5].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Save Image").min_size(Vec2::from([
                                ui.available_width() * 0.95,
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
            // TODO: Implement this
            // let (id, rect) = ui.allocate_space(ui.available_size());
            // let response = ui.interact(rect, id, egui::Sense::click_and_drag());
            // // Allow dragging the background as well.
            // if response.dragged() {
            //     app.transform.translation += response.drag_delta();
            // }
            // 
            // // Plot-like reset
            // if response.double_clicked() {
            //     app.transform = TSTransform::default();
            // }
            // 
            // let transform =
            //     TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * app.transform;
            // 
            // if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            //     // Note: doesn't catch zooming / panning if a button in this PanZoom container is hovered.
            //     if response.hovered() {
            //         let pointer_in_layer = transform.inverse() * pointer;
            //         let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
            //         let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
            // 
            //         // Zoom in on pointer:
            //         app.transform = app.transform
            //             * TSTransform::from_translation(pointer_in_layer.to_vec2())
            //             * TSTransform::from_scaling(zoom_delta)
            //             * TSTransform::from_translation(-pointer_in_layer.to_vec2());
            // 
            //         // Pan:
            //         app.transform = TSTransform::from_translation(pan_delta) * app.transform;
            //     }
            // }
            ui.allocate_ui_with_layout(
                [2. * ui.available_width() / 3., ui.available_height()].into(),
                egui::Layout::top_down(egui::Align::TOP),
                |ui| {
                    ui.vertical_centered_justified(|ui| {
                        let img = match app.image_type {
                            0 => {&app.orig_img},
                            1 => {&app.ascii_img.as_ref().unwrap()},
                            2 => {&app.sobel_img.as_ref().unwrap()},
                            3 => {&app.gaus_img.as_ref().unwrap()},
                            _ => {panic!("Invalid image type")},
                        };
                        ui
                            .add_sized(
                                Vec2::new(ui.available_width() * 0.9, ui.available_height() * 0.9),
                                egui::Image::new(ui::utils::convert_image_to_texture(
                                    img,
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
                        ui.add(egui::Slider::new(&mut app.gamma, 0.1..=3.0).text("Gamma"));

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