use eframe::egui;
use std::time::{Duration, Instant};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

mod rules {
    pub mod assimilation;
    pub mod conway;
    pub mod count;
    pub mod dayandnight;
    pub mod diamoeba;
    pub mod dotlife;
    pub mod drylife;
    pub mod highlife;
    pub mod honeylife;
    pub mod invertamaze;
    pub mod lifewithoutdeath;
    pub mod vote;
}

const GRID_SIZE: usize = 100;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rules {
    Assimilation,
    Conway,
    DayAndNight,
    Diamoeba,
    DotLife,
    DryLife,
    Highlife,
    HoneyLife,
    InvertaMaze,
    LifeWithoutDeath,
    Vote,
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
    url: String,
    description: String,
    rule: String,
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
            rule: rules::conway::RULE.to_string(),
            url: rules::conway::URL.to_string(),
            description: rules::conway::DESCRIPTION.to_string(),
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
            Rules::Assimilation => {
                self.grid = rules::assimilation::update_grid(&self.grid);
                self.url = rules::assimilation::URL.to_string();
                self.description = rules::assimilation::DESCRIPTION.to_string();
                self.rule = rules::assimilation::RULE.to_string();
            },
            Rules::Conway => {
                self.grid = rules::conway::update_grid(&self.grid);
                self.url = rules::conway::URL.to_string();
                self.description = rules::conway::DESCRIPTION.to_string();
                self.rule = rules::conway::RULE.to_string();
            },
            Rules::DayAndNight => {
                self.grid = rules::dayandnight::update_grid(&self.grid);
                self.url = rules::dayandnight::URL.to_string();
                self.description = rules::dayandnight::DESCRIPTION.to_string();
                self.rule = rules::dayandnight::RULE.to_string();
            },
            Rules::Diamoeba => {
                self.grid = rules::diamoeba::update_grid(&self.grid);
                self.url = rules::diamoeba::URL.to_string();
                self.description = rules::diamoeba::DESCRIPTION.to_string();
                self.rule = rules::diamoeba::RULE.to_string();
            },
            Rules::DryLife => {
                self.grid = rules::drylife::update_grid(&self.grid);
                self.url = rules::drylife::URL.to_string();
                self.description = rules::drylife::DESCRIPTION.to_string();
                self.rule = rules::drylife::RULE.to_string();
            },
            Rules::DotLife => {
                self.grid = rules::dotlife::update_grid(&self.grid);
                self.url = rules::dotlife::URL.to_string();
                self.description = rules::dotlife::DESCRIPTION.to_string();
                self.rule = rules::dotlife::RULE.to_string();
            },
            Rules::Highlife => {
                self.grid = rules::highlife::update_grid(&self.grid);
                self.url = rules::highlife::URL.to_string();
                self.description = rules::highlife::DESCRIPTION.to_string();
                self.rule = rules::highlife::RULE.to_string();
            },
            Rules::HoneyLife => {
                self.grid = rules::honeylife::update_grid(&self.grid);
                self.url = rules::honeylife::URL.to_string();
                self.description = rules::honeylife::DESCRIPTION.to_string();
                self.rule = rules::honeylife::RULE.to_string();
            },
            Rules::InvertaMaze => {
                self.grid = rules::invertamaze::update_grid(&self.grid);
                self.url = rules::invertamaze::URL.to_string();
                self.description = rules::invertamaze::DESCRIPTION.to_string();
                self.rule = rules::invertamaze::RULE.to_string();
            },
            Rules::LifeWithoutDeath => {
                self.grid = rules::lifewithoutdeath::update_grid(&self.grid);
                self.url = rules::lifewithoutdeath::URL.to_string();
                self.description = rules::lifewithoutdeath::DESCRIPTION.to_string();
                self.rule = rules::lifewithoutdeath::RULE.to_string();
            },
            Rules::Vote => {
                self.grid = rules::vote::update_grid(&self.grid);
                self.url = rules::vote::URL.to_string();
                self.description = rules::vote::DESCRIPTION.to_string();
                self.rule = rules::vote::RULE.to_string();
            },
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
                ui.horizontal(|ui| {
                    ui.label("Rule Set");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.rule_ui))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.rule_ui, Rules::Conway, "Conway");
                            ui.selectable_value(&mut self.rule_ui, Rules::Assimilation, "Assimilation");
                            ui.selectable_value(&mut self.rule_ui, Rules::DayAndNight, "Day and Night");
                            ui.selectable_value(&mut self.rule_ui, Rules::Diamoeba, "Diamoeba");
                            ui.selectable_value(&mut self.rule_ui, Rules::DotLife, "Dot Life");
                            ui.selectable_value(&mut self.rule_ui, Rules::DryLife, "Dry Life");
                            ui.selectable_value(&mut self.rule_ui, Rules::Highlife, "High Life");
                            ui.selectable_value(&mut self.rule_ui, Rules::HoneyLife, "Honey Life");
                            ui.selectable_value(&mut self.rule_ui, Rules::InvertaMaze, "Inverta Maze");
                            ui.selectable_value(&mut self.rule_ui, Rules::LifeWithoutDeath, "Life Without Death");
                            ui.selectable_value(&mut self.rule_ui, Rules::Vote, "Vote");
                        }
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Rule");
                    ui.label(self.rule.to_string());
                });
                ui.horizontal(|ui| {
                    ui.label("Seed");
                    ui.add(egui::TextEdit::singleline(&mut self.seed_ui).font(egui::TextStyle::Monospace));
                });
                ui.horizontal(|ui| {
                    ui.label("Alive Cell %");
                    ui.add(egui::Slider::new(&mut self.percent_live_cell, 1.0..=100.0));
                });
                ui.horizontal(|ui| {
                    if ui.button("Randomize Seed").clicked() {
                        let mut rng = rand::thread_rng();
                        self.seed = rng.gen();
                        self.seed_ui = self.seed.to_string();
                        self.generate_grid();
                    }
                    if ui.button("Regenerate").clicked() {
                        if let Ok(value) = self.seed_ui.parse() {
                            self.seed = value;
                        }
                        self.seed_ui = self.seed.to_string();
                        self.generate_grid();
                    }
                });
                ui.separator();
                ui.label(self.description.to_string());
                ui.hyperlink_to("Rule Wiki Page", self.url.to_string());

                ui.separator();
                ui.label("UI Settings");
                ui.horizontal(|ui| {
                    ui.label("Cell Size");
                    ui.add(egui::Slider::new(&mut self.cell_size, 1.0..=10.0));
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
