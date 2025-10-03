use crate::{Color, ColorScheme, Simulation2D};
use rand::Rng;

pub struct Sandpile {
    pub grid_width: usize,
    pub grid_height: usize,
    pub drop_rate: f32,
    pub critical_mass: u8,
    pub color_scheme: ColorScheme,
    pub show_avalanches: bool,
    pub drop_mode: DropMode,

    grid: Vec<u8>,
    avalanche_sites: Vec<bool>,  // Track recent avalanches
    time_accumulator: f32,
    total_drops: usize,
    total_avalanches: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DropMode {
    Center,     // Drop in center
    Random,     // Drop randomly
    Pattern,    // Drop in a pattern
}

impl DropMode {
    pub fn all() -> Vec<DropMode> {
        vec![
            DropMode::Center,
            DropMode::Random,
            DropMode::Pattern,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            DropMode::Center => "Center",
            DropMode::Random => "Random",
            DropMode::Pattern => "Pattern",
        }
    }
}

impl Default for Sandpile {
    fn default() -> Self {
        let width = 150;
        let height = 150;

        Self {
            grid_width: width,
            grid_height: height,
            drop_rate: 10.0,
            critical_mass: 4,
            color_scheme: ColorScheme::Fire,
            show_avalanches: true,
            drop_mode: DropMode::Center,
            grid: vec![0; width * height],
            avalanche_sites: vec![false; width * height],
            time_accumulator: 0.0,
            total_drops: 0,
            total_avalanches: 0,
        }
    }
}

impl Sandpile {
    pub fn new() -> Self {
        Self::default()
    }

    fn drop_sand(&mut self) {
        let mut rng = rand::thread_rng();

        let (x, y) = match self.drop_mode {
            DropMode::Center => (self.grid_width / 2, self.grid_height / 2),
            DropMode::Random => (
                rng.gen_range(1..self.grid_width - 1),
                rng.gen_range(1..self.grid_height - 1),
            ),
            DropMode::Pattern => {
                // Spiral pattern
                let t = self.total_drops as f32 * 0.1;
                let radius = (t * 0.1).min(self.grid_width as f32 / 3.0);
                let angle = t * 0.5;
                let cx = self.grid_width / 2;
                let cy = self.grid_height / 2;
                let x = (cx as f32 + radius * angle.cos()) as usize;
                let y = (cy as f32 + radius * angle.sin()) as usize;
                (
                    x.clamp(1, self.grid_width - 2),
                    y.clamp(1, self.grid_height - 2),
                )
            }
        };

        let idx = y * self.grid_width + x;
        self.grid[idx] += 1;
        self.total_drops += 1;

        // Trigger avalanche if needed
        self.avalanche();
    }

    fn avalanche(&mut self) {
        // Clear previous avalanche markers
        self.avalanche_sites.fill(false);

        let mut any_toppled = true;
        let mut avalanche_occurred = false;

        while any_toppled {
            any_toppled = false;

            for y in 1..(self.grid_height - 1) {
                for x in 1..(self.grid_width - 1) {
                    let idx = y * self.grid_width + x;

                    if self.grid[idx] >= self.critical_mass {
                        // Topple
                        self.grid[idx] -= self.critical_mass;

                        // Distribute to neighbors
                        self.grid[idx - 1] += 1;  // left
                        self.grid[idx + 1] += 1;  // right
                        self.grid[idx - self.grid_width] += 1;  // up
                        self.grid[idx + self.grid_width] += 1;  // down

                        self.avalanche_sites[idx] = true;
                        any_toppled = true;
                        avalanche_occurred = true;
                    }
                }
            }
        }

        if avalanche_occurred {
            self.total_avalanches += 1;
        }
    }

    pub fn reset(&mut self) {
        // Resize grids if dimensions changed
        let required_size = self.grid_width * self.grid_height;
        if self.grid.len() != required_size {
            self.grid = vec![0; required_size];
            self.avalanche_sites = vec![false; required_size];
        } else {
            self.grid.fill(0);
            self.avalanche_sites.fill(false);
        }

        self.time_accumulator = 0.0;
        self.total_drops = 0;
        self.total_avalanches = 0;
    }
}

impl Simulation2D for Sandpile {
    fn name(&self) -> &str {
        "Sandpile Model"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(width * height);

        let cell_width = width / self.grid_width;
        let cell_height = height / self.grid_height;

        for py in 0..height {
            for px in 0..width {
                let gx = px / cell_width.max(1);
                let gy = py / cell_height.max(1);

                if gx < self.grid_width && gy < self.grid_height {
                    let idx = gy * self.grid_width + gx;
                    let sand = self.grid[idx];

                    // Show avalanche sites in bright color
                    if self.show_avalanches && self.avalanche_sites[idx] {
                        colors.push(Color::from_rgb(255, 255, 0));
                    } else {
                        // Color by sand amount
                        let t = (sand as f32 / self.critical_mass as f32).clamp(0.0, 1.0);
                        colors.push(self.color_scheme.map(t, true));
                    }
                } else {
                    colors.push(Color::BLACK);
                }
            }
        }

        colors
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Sandpile Model");

        ui.label(format!("Drops: {}", self.total_drops));
        ui.label(format!("Avalanches: {}", self.total_avalanches));

        if self.total_drops > 0 {
            let avalanche_rate = self.total_avalanches as f32 / self.total_drops as f32;
            ui.label(format!("Avalanche rate: {:.1}%", avalanche_rate * 100.0));
        }

        ui.separator();

        egui::CollapsingHeader::new("⚙ Simulation Settings")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.drop_rate, 0.1..=100.0)
                    .logarithmic(true)
                    .text("Drops per second"));

                if ui.add(egui::Slider::new(&mut self.critical_mass, 3..=8)
                    .text("Critical Mass")).changed() {
                    changed = true;
                }

                egui::ComboBox::from_label("Drop Mode")
                    .selected_text(self.drop_mode.name())
                    .show_ui(ui, |ui| {
                        for mode in DropMode::all() {
                            if ui.selectable_value(&mut self.drop_mode, mode, mode.name()).clicked() {
                                changed = true;
                            }
                        }
                    });

                ui.label("Higher critical mass = larger");
                ui.label("avalanches before toppling");
            });

        egui::CollapsingHeader::new("🔍 Grid Settings")
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.grid_width, 50..=250)
                    .text("Grid Width")).changed() {
                    self.reset();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.grid_height, 50..=250)
                    .text("Grid Height")).changed() {
                    self.reset();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_avalanches, "Highlight Avalanches").changed();

                egui::ComboBox::from_label("Color Scheme")
                    .selected_text(self.color_scheme.name())
                    .show_ui(ui, |ui| {
                        for scheme in ColorScheme::all() {
                            if ui.selectable_value(&mut self.color_scheme, scheme, scheme.name()).clicked() {
                                changed = true;
                            }
                        }
                    });
            });

        if ui.button("🔄 Reset").clicked() {
            self.reset();
            changed = true;
        }

        egui::CollapsingHeader::new("📊 Info")
            .show(ui, |ui| {
                ui.label("The Abelian Sandpile Model is a");
                ui.label("classic example of self-organized");
                ui.label("criticality. Sand grains are added");
                ui.label("one at a time. When a pile gets too");
                ui.label("high, it topples to its neighbors,");
                ui.label("potentially triggering avalanches");
                ui.label("of all sizes - a power law!");
            });

        // Auto-dropping
        let dt = ui.input(|i| i.stable_dt);
        self.time_accumulator += dt * self.drop_rate;

        while self.time_accumulator >= 1.0 {
            self.drop_sand();
            self.time_accumulator -= 1.0;
            changed = true;
        }

        changed
    }
}
