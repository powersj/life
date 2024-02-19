use eframe::egui;
use egui::{Color32, Pos2, LayerId, Order, Id, Rect, Vec2};
use rand::Rng;
use std::time::{Duration, Instant};

const COLOR_LIVE: Color32 = Color32::BLACK;
const COLOR_DEAD: Color32 = Color32::WHITE;

struct GridApp {
    last_update: Instant,
    grid_size: usize,
    grid: Vec<Vec<Color32>>,
}

impl Default for GridApp {
    fn default() -> Self {
        let last_update = Instant::now();
        let grid_size = 50;
        let mut grid = vec![vec![COLOR_DEAD; grid_size]; grid_size]; // Initialize with default color
        let mut rng = rand::thread_rng();

        // Initialize grid with white color
        for row in 0..grid_size {
            for col in 0..grid_size {
                if rng.gen_range(0..2) == 1 {
                    grid[row][col] = COLOR_LIVE;
                } else {
                    grid[row][col] = COLOR_DEAD;
                }
            }
        }

        Self { last_update, grid_size, grid}
    }
}

impl GridApp {
    fn update_grid(&mut self) {
        let mut new_grid = vec![vec![COLOR_DEAD; self.grid_size]; self.grid_size];
        for x in 0..self.grid_size {
            for y in 0..self.grid_size {
                let mut count = 0;
                if x>=1 && self.grid[x-1][y] == COLOR_LIVE {
                    count += 1;
                }
                if x+1 < self.grid_size && self.grid[x+1][y] == COLOR_LIVE {
                    count += 1;
                }
                if y>=1 && self.grid[x][y-1] == COLOR_LIVE {
                    count += 1;
                }
                if y+1 < self.grid_size && self.grid[x][y+1] == COLOR_LIVE {
                    count += 1;
                }
                if x>=1 && y>=1 && self.grid[x-1][y-1] == COLOR_LIVE {
                    count += 1;
                }
                if x>=1 && y+1 < self.grid_size && self.grid[x-1][y+1] == COLOR_LIVE {
                    count += 1;
                }
                if x+1 < self.grid_size && y>=1 && self.grid[x+1][y-1] == COLOR_LIVE {
                    count += 1;
                }
                if x+1 < self.grid_size && y+1 < self.grid_size && self.grid[x+1][y+1] == COLOR_LIVE {
                    count += 1;
                }

                // Any live cell with fewer than two live neighbors dies, as if by under-population.
                // Any live cell with two or three live neighbors lives on to the next generation.
                // Any live cell with more than three live neighbors dies, as if by over-population.
                // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                if self.grid[x][y] == COLOR_LIVE && (count < 2 || count > 3){
                    new_grid[x][y] = COLOR_DEAD;
                } else if self.grid[x][y] == COLOR_DEAD && count == 3 {
                    new_grid[x][y] = COLOR_LIVE;
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
        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(200) {
            self.update_grid();
            self.last_update = now;
        }

        let side_panel_max_width =  150.0;

        egui::SidePanel::left("nav").max_width(side_panel_max_width).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Controls");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                for row in 0..self.grid_size {
                    for col in 0..self.grid_size {
                        let rect = Rect::from_min_size(Pos2::new(side_panel_max_width + (row as f32 * 10 as f32), col as f32 * 10 as f32), Vec2::new(10.0, 10.0));
                        //let (_, painter) = ui.allocate_painter(rect.size(), egui::Sense::click());
                        //painter.rect_filled(rect, 0.0, self.grid[row][col]);
                        ui.painter().rect_filled(rect, 0.0, self.grid[row][col]);
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
