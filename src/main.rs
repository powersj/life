use eframe::egui;
use std::time::{Duration, Instant};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

mod rules {
    pub mod conway;
    pub mod count;
    pub mod other;
}

const GRID_SIZE: usize = 100;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rules {
    Conway,
    Other,
}


#[derive(Debug)]
struct GridApp {
    grid: Vec<Vec<i32>>,
    cell_size: f32,
    cell_color_alive: egui::Color32,
    cell_color_empty: egui::Color32,
    percent_live_cell: f32,

    last_update: Instant,
    seed: u64,
    seed_ui: String,

    rule_ui: Rules,
}

impl Default for GridApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let initial_seed = rng.gen();

        Self {
            grid: vec![vec![0; GRID_SIZE]; GRID_SIZE],
            cell_size: 9.0,
            percent_live_cell: 10.0,
            cell_color_alive: egui::Color32::BLACK,
            cell_color_empty:egui::Color32::WHITE,

            last_update: Instant::now(),
            seed: initial_seed,
            seed_ui: initial_seed.to_string(),

            rule_ui: Rules::Conway,
        }
    }
}

impl GridApp {
    fn generate_grid(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if rng.gen_range(0..100) < self.percent_live_cell as i32 {
                    self.grid[row][col] = 1;
                } else {
                    self.grid[row][col] = 0;
                }
            }
        }
    }

    fn update_grid(&mut self) {
        match self.rule_ui {
            Rules::Conway => self.grid = rules::conway::update_grid(&self.grid),
            Rules::Other => self.grid = rules::other::update_grid(&self.grid),
        }
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(200) {
            self.update_grid();
            self.last_update = now;
        }

        egui::SidePanel::left("controls").exact_width(200.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Game of Life");
                ui.separator();
                egui::ComboBox::from_label("Rule Set")
                    .selected_text(format!("{:?}", self.rule_ui))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.rule_ui, Rules::Conway, "Conway");
                        ui.selectable_value(&mut self.rule_ui, Rules::Other, "Other");
                    }
                );
                ui.horizontal(|ui| {
                    ui.label("Seed");
                    ui.add(egui::TextEdit::singleline(&mut self.seed_ui).font(egui::TextStyle::Monospace));
                });

                ui.horizontal(|ui| {
                    if ui.button("Regenerate").clicked() {
                        if let Ok(value) = self.seed_ui.parse() {
                            self.seed = value;
                        }
                        self.seed_ui = self.seed.to_string();
                        self.generate_grid();
                    }
                    if ui.button("Randomize Seed").clicked() {
                        let mut rng = rand::thread_rng();
                        self.seed = rng.gen();
                        self.seed_ui = self.seed.to_string();
                        self.generate_grid();
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Cell Size");
                    ui.add(egui::Slider::new(&mut self.cell_size, 1.0..=10.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Alive Cell %");
                    ui.add(egui::Slider::new(&mut self.percent_live_cell, 1.0..=100.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Alive Cell Color");
                    ui.color_edit_button_srgba(&mut self.cell_color_alive);
                });
                ui.horizontal(|ui| {
                    ui.label("Empty Cell Color");
                    ui.color_edit_button_srgba(&mut self.cell_color_empty);
                });

                ui.separator();
            });
        });
        // get the width of the side panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for row in 0..GRID_SIZE {
                    for col in 0..GRID_SIZE {
                        let rect = egui::Rect::from_min_size(
                            egui::Pos2::new(265.0 + (row as f32 * 10.0), 30.0 + (col as f32 * 10.0)),
                            egui::Vec2::new(self.cell_size, self.cell_size)
                        );
                        if self.grid[row][col] == 0 {
                            ui.painter().rect_filled(rect, 0.0, self.cell_color_empty);
                        } else {
                            ui.painter().rect_filled(rect, 0.0, self.cell_color_alive);
                        }

                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut app = Box::<GridApp>::default();
    app.generate_grid();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1300.0, 1100.0]),
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "life",
        options,
        Box::new(|_cc| app),
    )
}
