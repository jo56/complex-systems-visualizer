use crate::{Color, ColorScheme, Simulation2D};
use rand::Rng;

pub struct CyclicCA {
    pub grid_width: usize,
    pub grid_height: usize,
    pub num_states: usize,
    pub threshold: usize,
    pub speed: f32,
    pub color_scheme: ColorScheme,
    pub neighborhood: Neighborhood,

    grid: Vec<u8>,
    next_grid: Vec<u8>,
    time_accumulator: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Neighborhood {
    VonNeumann,  // 4 neighbors (cardinal)
    Moore,       // 8 neighbors (including diagonals)
    Extended,    // 12 neighbors (extended Moore)
}

impl Neighborhood {
    pub fn all() -> Vec<Neighborhood> {
        vec![
            Neighborhood::VonNeumann,
            Neighborhood::Moore,
            Neighborhood::Extended,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Neighborhood::VonNeumann => "Von Neumann (4)",
            Neighborhood::Moore => "Moore (8)",
            Neighborhood::Extended => "Extended (12)",
        }
    }

    fn get_offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Neighborhood::VonNeumann => vec![
                (0, -1), (1, 0), (0, 1), (-1, 0),
            ],
            Neighborhood::Moore => vec![
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ],
            Neighborhood::Extended => vec![
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
                // Extended neighbors
                (0, -2), (2, 0), (0, 2), (-2, 0),
            ],
        }
    }
}

impl Default for CyclicCA {
    fn default() -> Self {
        let width = 200;
        let height = 150;

        let mut rng = rand::thread_rng();
        let grid: Vec<u8> = (0..width * height)
            .map(|_| rng.gen_range(0..14))
            .collect();

        Self {
            grid_width: width,
            grid_height: height,
            num_states: 14,
            threshold: 3,
            speed: 10.0,
            color_scheme: ColorScheme::Rainbow,
            neighborhood: Neighborhood::Moore,
            grid,
            next_grid: vec![0; width * height],
            time_accumulator: 0.0,
        }
    }
}

impl CyclicCA {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let idx = y * self.grid_width + x;
                let current_state = self.grid[idx];
                let next_state = (current_state + 1) % self.num_states as u8;

                // Count neighbors in next state
                let mut count = 0;
                for (dx, dy) in self.neighborhood.get_offsets() {
                    let nx = (x as i32 + dx + self.grid_width as i32) % self.grid_width as i32;
                    let ny = (y as i32 + dy + self.grid_height as i32) % self.grid_height as i32;
                    let nidx = ny as usize * self.grid_width + nx as usize;

                    if self.grid[nidx] == next_state {
                        count += 1;
                    }
                }

                // Update if threshold met
                if count >= self.threshold {
                    self.next_grid[idx] = next_state;
                } else {
                    self.next_grid[idx] = current_state;
                }
            }
        }

        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in &mut self.grid {
            *cell = rng.gen_range(0..self.num_states as u8);
        }
        self.time_accumulator = 0.0;
    }

    pub fn reset_with_seed(&mut self, seed: &str) {
        self.grid.fill(0);

        match seed {
            "spiral" => {
                // Create a spiral seed
                let cx = self.grid_width / 2;
                let cy = self.grid_height / 2;
                for i in 0..self.num_states {
                    let angle = (i as f32 / self.num_states as f32) * std::f32::consts::TAU;
                    let radius = 10.0;
                    let x = (cx as f32 + radius * angle.cos()) as usize;
                    let y = (cy as f32 + radius * angle.sin()) as usize;
                    if x < self.grid_width && y < self.grid_height {
                        let idx = y * self.grid_width + x;
                        self.grid[idx] = i as u8;
                    }
                }
            }
            "stripes" => {
                // Create vertical stripes
                for y in 0..self.grid_height {
                    for x in 0..self.grid_width {
                        let idx = y * self.grid_width + x;
                        self.grid[idx] = (x * self.num_states / self.grid_width) as u8;
                    }
                }
            }
            "corners" => {
                // Different states in each corner
                let half_w = self.grid_width / 2;
                let half_h = self.grid_height / 2;
                for y in 0..self.grid_height {
                    for x in 0..self.grid_width {
                        let idx = y * self.grid_width + x;
                        let state = if x < half_w && y < half_h {
                            0
                        } else if x >= half_w && y < half_h {
                            self.num_states / 4
                        } else if x < half_w && y >= half_h {
                            self.num_states / 2
                        } else {
                            self.num_states * 3 / 4
                        };
                        self.grid[idx] = state as u8;
                    }
                }
            }
            _ => self.reset(),
        }
    }
}

impl Simulation2D for CyclicCA {
    fn name(&self) -> &str {
        "Cyclic Cellular Automaton"
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
                    let state = self.grid[gy * self.grid_width + gx];
                    let t = state as f32 / self.num_states as f32;
                    colors.push(self.color_scheme.map(t, true));
                } else {
                    colors.push(Color::BLACK);
                }
            }
        }

        colors
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Cyclic Cellular Automaton");

        egui::CollapsingHeader::new("⚙ Automaton Rules")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.num_states, 3..=24)
                    .text("Number of States")).changed() {
                    self.reset();
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.threshold, 1..=8)
                    .text("Threshold")).changed();

                egui::ComboBox::from_label("Neighborhood")
                    .selected_text(self.neighborhood.name())
                    .show_ui(ui, |ui| {
                        for n in Neighborhood::all() {
                            if ui.selectable_value(&mut self.neighborhood, n, n.name()).clicked() {
                                changed = true;
                            }
                        }
                    });

                ui.label("Higher threshold = slower propagation");
                ui.label("More states = more spiral arms");
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.grid_width, 50..=400)
                    .text("Grid Width")).changed() {
                    self.reset();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.grid_height, 50..=300)
                    .text("Grid Height")).changed() {
                    self.reset();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=60.0)
                    .logarithmic(true)
                    .text("Steps per second"));

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

        egui::CollapsingHeader::new("🎯 Initial Conditions")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Random").clicked() {
                        self.reset();
                        changed = true;
                    }
                    if ui.button("Spiral").clicked() {
                        self.reset_with_seed("spiral");
                        changed = true;
                    }
                    if ui.button("Stripes").clicked() {
                        self.reset_with_seed("stripes");
                        changed = true;
                    }
                    if ui.button("Corners").clicked() {
                        self.reset_with_seed("corners");
                        changed = true;
                    }
                });
            });

        egui::CollapsingHeader::new("📊 Info")
            .show(ui, |ui| {
                ui.label("Cyclic CA simulates competing");
                ui.label("states in a 'rock-paper-scissors'");
                ui.label("dynamic, creating beautiful spirals");
                ui.label("and waves. Each state can be");
                ui.label("'consumed' by the next state.");
            });

        // Auto-stepping
        let dt = ui.input(|i| i.stable_dt);
        self.time_accumulator += dt * self.speed;

        while self.time_accumulator >= 1.0 {
            self.step();
            self.time_accumulator -= 1.0;
            changed = true;
        }

        changed
    }
}
