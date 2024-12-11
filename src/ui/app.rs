use std::path::PathBuf;
use eframe::egui;
use eframe::emath::TSTransform;
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
    pub edge_threshold: i32,
    pub tau: f32,
    pub gamma: f32,
    pub scale_factor_id: i32,
    pub image_type: usize,
    pub ascii_img: Option<DynamicImage>,
    pub sobel_img: Option<DynamicImage>,
    pub gaus_img: Option<DynamicImage>,
    pub edges: Option<Vec<Vec<usize>>>,
    pub changed: bool,
    pub up_scale_factor_id: i32,
    pub scale_factors: Vec<i32>,
    pub up_scale_factors: Vec<i32>,
    pub charset: Vec<String>,
    pub charset_text: String,
    pub transform: TSTransform
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
            edge_threshold: 4,
            tau: 0.9,
            gamma: 2.5,
            scale_factor_id: 0,
            image_type: 0,
            ascii_img: None,
            sobel_img: None,
            gaus_img: None,
            changed: true,
            edges: None,
            up_scale_factor_id: 0,
            scale_factors: vec![2, 4, 8, 16],
            up_scale_factors: vec![1, 2, 4, 8, 16],
            charset: [" ", ".", ";", "c", "o", "P", "O", "?", "@", "■"].iter().map(|&x| x.to_string()).collect(),
            charset_text: String::from(" , ., ;, c, o, P, O, ?, @, ■"),
            transform: TSTransform::default()
        }
    }
}

// Add a scaling option, to scale up
// Add auto format, so if you change name, format is still image
// Change font button

impl AsciiApp {
    pub fn update_images(&mut self, i: u32) {
        match i {
            0 => {},
            1 => {
                self.gen_gaus();
                self.gen_sobel();
                self.gen_ascii();
            },
            2 => {
                self.gen_gaus();
                self.gen_sobel();
            },
            3 => {
                self.gen_gaus();
            },
            _ => {},
        }
        self.changed = false;
    }
    
    fn gen_gaus(&mut self) {
        if self.changed || self.gaus_img.is_none() {
            let gaus = filters::gaussian::gaussian_diff(&self.orig_img, self.sigma_one as f32, self.sigma_two as f32, self.threshold, self.tau);
            self.gaus_img = Some(DynamicImage::ImageRgb8(gaus));
            self.sobel_img = None;
            self.ascii_img = None;
        }
    }
    
    fn gen_sobel(&mut self) {
        if self.changed || self.sobel_img.is_none() {
            let (sobel, edges) = filters::edge_detect::edge_filter(&self.gaus_img.clone().unwrap(), &self.orig_img, self.get_scale_factor() as u32, self.edge_threshold);
            self.sobel_img = Some(DynamicImage::ImageRgb8(sobel));
            self.edges = Some(edges);
            self.ascii_img = None;
        }
    }
    
    fn gen_ascii(&mut self) {
        if self.changed || self.ascii_img.is_none() {
            let ascii = filters::ascii::to_ascii_image(&self.orig_img.clone(), self.get_scale_factor() as u32, &self.edges.clone().unwrap(), &self.charset, self.get_upscale_factor(), self.gamma);
            self.ascii_img = Some(DynamicImage::ImageRgb8(ascii));
        }
    }

    fn get_scale_factor(&self) -> i32 {
        self.scale_factors[self.scale_factor_id as usize]
    }

    pub fn get_upscale_factor(&self) -> u32 {
        self.up_scale_factors[self.up_scale_factor_id as usize] as u32
    }

    pub fn check_charset_correctness(&mut self) -> bool {
        let split_commas_remove_space: Vec<String> = self.charset_text
            .split(',')
            .map(|x| if x.trim().is_empty() || x.chars().all(|c| c.is_whitespace()) {
                " ".to_string()  // Single space for any number of spaces
            } else {
                x.trim().to_string()
            }).collect();
        if split_commas_remove_space.iter().all(|x| x.chars().count() == 1) {
            self.charset = split_commas_remove_space;
            true
        } else {
            false
        }
    }
}

impl eframe::App for AsciiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::ui::top_panel(ctx);
        ui::ui::bottom_panel(ctx, self);
        ui::ui::central_panel(ctx, self);

        ui::utils::preview_files_being_dropped(ctx, self);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.picked_path = Some(i.raw.dropped_files[0].path.clone().unwrap());
            }
        });

        self.toasts.show(ctx);
    }
}