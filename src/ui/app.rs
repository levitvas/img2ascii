use std::path::PathBuf;
use eframe::egui;
use egui_notify::Anchor::BottomRight;
use egui_notify::Toasts;
use image::DynamicImage;

use crate::ui;
use crate::filters;

#[derive(Default)]
pub struct AsciiApp {
    pub picked_path: Option<PathBuf>,
    pub orig_img: image::DynamicImage,
    pub toasts: Toasts,
    pub sigma_one: i32,
    pub sigma_two: i32,
    pub threshold: i32,
    pub tau: f32,
    pub scale_factor_id: i32,
    pub image_type: usize,
    pub ascii_img: Option<image::DynamicImage>,
    pub sobel_img: Option<image::DynamicImage>,
    pub gaus_img: Option<image::DynamicImage>,
    pub edges: Option<Vec<Vec<usize>>>,
    pub changed: bool,
    pub scale_factors: Vec<i32>,
}

impl AsciiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ui::utils::configure_font(&cc.egui_ctx);
        Self {
            picked_path: Some(PathBuf::from("images\\pipe.jpg")),
            orig_img: image::open("images\\pipe.jpg").unwrap(),
            toasts: Toasts::default().with_anchor(BottomRight),
            sigma_one: 7,
            sigma_two: 20,
            threshold: 10,
            tau: 0.9,
            scale_factor_id: 0,
            image_type: 0,
            ascii_img: None,
            sobel_img: None,
            gaus_img: None,
            changed: true,
            edges: None,
            scale_factors: vec![2, 4, 8, 16],
        }
    }
}

impl AsciiApp {
    pub fn update_images(&mut self, i: u32) {
        match i {
            0 => {self.orig_img = image::open(self.picked_path.clone().unwrap()).unwrap();},
            1 => {
                if self.changed || self.ascii_img.is_none() {
                    if self.changed || self.edges.is_none() {
                        if self.changed || self.gaus_img.is_none() {
                            let gaus = filters::gaussian::gaussian_diff(&self.orig_img, self.sigma_one as f32, self.sigma_two as f32, self.threshold, self.tau);
                            self.gaus_img = Some(DynamicImage::ImageRgb8(gaus));
                        };
                        if self.changed || self.sobel_img.is_none() {
                            let (sobel, edges) = filters::edge_detect::edge_filter(&self.gaus_img.clone().unwrap(), &self.orig_img, self.get_scale_factor() as u32);
                            self.sobel_img = Some(DynamicImage::ImageRgb8(sobel));
                            self.edges = Some(edges);
                        }
                    }
                    self.ascii_img = Some(DynamicImage::ImageRgb8(filters::ascii::to_ascii_image(&self.orig_img, self.get_scale_factor() as u32, &self.edges.clone().unwrap())));
                }
            },
            2 => {
                if self.changed || self.sobel_img.is_none() {
                    if self.changed || self.gaus_img.is_none() {
                        let gaus = filters::gaussian::gaussian_diff(&self.orig_img, self.sigma_one as f32, self.sigma_two as f32, self.threshold, self.tau);
                        self.gaus_img = Some(DynamicImage::ImageRgb8(gaus));
                    };

                    let (sobel, edges) = filters::edge_detect::edge_filter(&self.gaus_img.clone().unwrap(), &self.orig_img, self.get_scale_factor() as u32);
                    self.sobel_img = Some(DynamicImage::ImageRgb8(sobel));
                    self.edges = Some(edges);
                }
            },
            3 => {
                if self.changed || self.gaus_img.is_none() {
                    let gaus = filters::gaussian::gaussian_diff(&self.orig_img, self.sigma_one as f32, self.sigma_two as f32, self.threshold, self.tau);
                    self.gaus_img = Some(DynamicImage::ImageRgb8(gaus));
                }
            },
            _ => todo!(),
        }
        self.changed = false;
    }

    fn get_scale_factor(&self) -> i32 {
        self.scale_factors[self.scale_factor_id as usize]
    }
}

impl eframe::App for AsciiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::ui::top_panel(ctx);
        ui::ui::bottom_panel(ctx, self);
        ui::ui::central_panel(ctx, self);

        ui::utils::preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.picked_path = Some(i.raw.dropped_files[0].path.clone().unwrap());
            }
        });

        self.toasts.show(ctx);
    }
}