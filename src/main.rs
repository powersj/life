use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use rand::Rng;

use std::thread;
use std::time::Duration;

struct GridApp {
    grid_size: usize,
    grid_colors: Vec<Vec<Color32>>,
}

impl Default for GridApp {
    fn default() -> Self {
        let grid_size = 50;
        let mut grid_colors = vec![vec![Color32::WHITE; grid_size]; grid_size]; // Initialize with default color

        // Initialize grid with white color
        for row in 0..grid_size {
            for col in 0..grid_size {
                grid_colors[row][col] = Color32::from_rgba_unmultiplied(
                    255,
                    255,
                    255,
                    255,
                );
            }
        }

        Self { grid_size, grid_colors }
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for row in 0..self.grid_size {
                for col in 0..self.grid_size {
                    let rect = Rect::from_min_size(Pos2::new(row as f32 * 10.0, col as f32 * 10.0), Vec2::new(10.0, 10.0));
                    ui.painter().rect_filled(rect, 0.0, self.grid_colors[row][col]);
                }
            }
        });

        //thread::sleep(Duration::from_millis(250));
        let mut rng = rand::thread_rng();
        // TODO: update random value with random color
        self.grid_colors[rng.gen_range(0..self.grid_size)][rng.gen_range(0..self.grid_size)] = Color32::from_rgba_unmultiplied(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            255,
        );
    }
}

fn main()  -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "life",
        options,
        Box::new(|_cc| Box::<GridApp>::default()),
    )
}
