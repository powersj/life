use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use egui::color_picker::Alpha;
use std::time::{Duration, Instant};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;



struct GridApp {
    last_update: Instant,
    grid_size: usize,
    cell_size: f32,
    grid: Vec<Vec<i32>>,
    seed: u64,
    seed_picker: String,
    color_live: Color32,
    color_dead: Color32,
}

impl Default for GridApp {
    fn default() -> Self {
        let last_update = Instant::now();
        let grid_size = 50;
        let cell_size = 9.0;
        let color_live: Color32 = Color32::BLACK;
        let color_dead: Color32 = Color32::WHITE;

        // Random seed at each start
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let seed_picker = seed.to_string();
        let mut rng = StdRng::seed_from_u64(seed);

        // Initialize grid with white color
        let mut grid = vec![vec![0; grid_size]; grid_size];
        for row in 0..grid_size {
            for col in 0..grid_size {
                if rng.gen_range(0..2) == 1 {
                    grid[row][col] = 1;
                } else {
                    grid[row][col] = 0;
                }
            }
        }

        Self { last_update, grid_size, cell_size, grid, seed, color_live, color_dead, seed_picker}
    }
}

impl GridApp {
    fn update_grid(&mut self) {
        let mut new_grid = vec![vec![0; self.grid_size]; self.grid_size];
        for x in 0..self.grid_size {
            for y in 0..self.grid_size {
                let mut count = 0;
                if x>=1 && self.grid[x-1][y] == 1 {
                    count += 1;
                }
                if x+1 < self.grid_size && self.grid[x+1][y] == 1 {
                    count += 1;
                }
                if y>=1 && self.grid[x][y-1] == 1 {
                    count += 1;
                }
                if y+1 < self.grid_size && self.grid[x][y+1] == 1 {
                    count += 1;
                }
                if x>=1 && y>=1 && self.grid[x-1][y-1] == 1 {
                    count += 1;
                }
                if x>=1 && y+1 < self.grid_size && self.grid[x-1][y+1] == 1 {
                    count += 1;
                }
                if x+1 < self.grid_size && y>=1 && self.grid[x+1][y-1] == 1 {
                    count += 1;
                }
                if x+1 < self.grid_size && y+1 < self.grid_size && self.grid[x+1][y+1] == 1 {
                    count += 1;
                }

                // Any live cell with fewer than two live neighbors dies, as if by under-population.
                // Any live cell with two or three live neighbors lives on to the next generation.
                // Any live cell with more than three live neighbors dies, as if by over-population.
                // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                if self.grid[x][y] == 1 && (count < 2 || count > 3){
                    new_grid[x][y] = 0;
                } else if self.grid[x][y] == 0 && count == 3 {
                    new_grid[x][y] = 1;
                } else {
                    new_grid[x][y] = self.grid[x][y];
                }
            }
        }
        self.grid = new_grid;
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(200) {
            self.update_grid();
            self.last_update = now;
        }

        egui::SidePanel::left("controls").exact_width(200.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Game of Life");
                ui.separator();
                ui.label("Settings");
                ui.label("Live Color");
                egui::color_picker::color_picker_color32(ui, &mut self.color_dead, Alpha::Opaque);
                ui.label("Empty Color");
                egui::color_picker::color_picker_color32(ui, &mut self.color_live, Alpha::Opaque);
                ui.add(egui::Slider::new(&mut self.cell_size, 1.0..=10.0).text("Cell Size"));
                ui.separator();

                ui.add(egui::TextEdit::singleline(&mut self.seed_picker));

                ui.label("Seed");
                if ui.button("Regenerate").clicked() {
                    if let Ok(value) = self.seed_picker.parse() {
                        self.seed = value;
                    }
                    self.seed_picker = self.seed.to_string();

                    let mut rng = StdRng::seed_from_u64(self.seed);
                    for row in 0..self.grid_size {
                        for col in 0..self.grid_size {
                            if rng.gen_range(0..2) == 1 {
                                self.grid[row][col] = 1;
                            } else {
                                self.grid[row][col] = 0;
                            }
                        }
                    }
                }
                if ui.button("Random Seed").clicked() {
                    let mut rng = rand::thread_rng();
                    self.seed = rng.gen();
                    self.seed_picker = self.seed.to_string();
                    let mut rng = StdRng::seed_from_u64(self.seed);
                    for row in 0..self.grid_size {
                        for col in 0..self.grid_size {
                            if rng.gen_range(0..3) == 1 {
                                self.grid[row][col] = 1;
                            } else {
                                self.grid[row][col] = 0;
                            }
                        }
                    }
                }
            });
        });
        // get the width of the side panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for row in 0..self.grid_size {
                    for col in 0..self.grid_size {
                        let rect = Rect::from_min_size(
                            Pos2::new(250.0 + (row as f32 * 10.0), 30.0 + (col as f32 * 10.0)),
                            Vec2::new(self.cell_size, self.cell_size)
                        );
                        if self.grid[row][col] == 0 {
                            ui.painter().rect_filled(rect, 0.0, self.color_dead);
                        } else {
                            ui.painter().rect_filled(rect, 0.0, self.color_live);
                        }

                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 1080.0]),
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "life",
        options,
        Box::new(|_cc| Box::<GridApp>::default()),
    )
}
