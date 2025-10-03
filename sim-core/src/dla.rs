use crate::{Color, ColorScheme, Simulation2D};
use rand::Rng;

pub struct DLA {
    pub grid_width: usize,
    pub grid_height: usize,
    pub num_particles: usize,
    pub stickiness: f32,
    pub spawn_radius_ratio: f32,
    pub color_by_age: bool,
    pub color_scheme: ColorScheme,
    pub particle_size: f32,

    grid: Vec<Option<usize>>,  // Some(age) if particle stuck, None if empty
    particles_stuck: usize,
    max_radius: f32,
    paused: bool,
}

impl Default for DLA {
    fn default() -> Self {
        let width = 256;
        let height = 256;
        let mut grid = vec![None; width * height];

        // Seed with center particle
        grid[height / 2 * width + width / 2] = Some(0);

        Self {
            grid_width: width,
            grid_height: height,
            num_particles: 5000,
            stickiness: 1.0,
            spawn_radius_ratio: 0.8,
            color_by_age: true,
            color_scheme: ColorScheme::Ice,
            particle_size: 1.5,
            grid,
            particles_stuck: 1,
            max_radius: 1.0,
            paused: false,
        }
    }
}

impl DLA {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) -> bool {
        if self.paused || self.particles_stuck >= self.num_particles {
            return false;
        }

        let mut rng = rand::thread_rng();

        // Spawn radius - outside the structure
        let spawn_radius = (self.max_radius + 10.0).max(50.0);
        let center_x = self.grid_width as f32 / 2.0;
        let center_y = self.grid_height as f32 / 2.0;

        // Spawn particle on a circle
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let mut x = center_x + spawn_radius * angle.cos();
        let mut y = center_y + spawn_radius * angle.sin();

        // Random walk until it sticks or escapes
        for _ in 0..10000 {
            // Check if we've gone too far
            let dx = x - center_x;
            let dy = y - center_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > spawn_radius * 2.0 {
                // Escaped, restart
                return true;
            }

            // Check if next to a stuck particle
            let ix = x as usize;
            let iy = y as usize;

            if ix > 0 && ix < self.grid_width - 1 && iy > 0 && iy < self.grid_height - 1 {
                let mut should_stick = false;

                // Check 8 neighbors
                for dy in -1..=1i32 {
                    for dx in -1..=1i32 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (ix as i32 + dx) as usize;
                        let ny = (iy as i32 + dy) as usize;
                        let nidx = ny * self.grid_width + nx;

                        if self.grid[nidx].is_some() {
                            // Neighbor is stuck, maybe stick here
                            if rng.gen::<f32>() < self.stickiness {
                                should_stick = true;
                                break;
                            }
                        }
                    }
                    if should_stick {
                        break;
                    }
                }

                if should_stick {
                    // Stick here
                    let idx = iy * self.grid_width + ix;
                    self.grid[idx] = Some(self.particles_stuck);
                    self.particles_stuck += 1;

                    // Update max radius
                    let dx = ix as f32 - center_x;
                    let dy = iy as f32 - center_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    self.max_radius = self.max_radius.max(dist);

                    return true;
                }
            }

            // Random walk step
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            x += 2.0 * angle.cos();
            y += 2.0 * angle.sin();

            // Clamp to bounds
            x = x.clamp(1.0, self.grid_width as f32 - 2.0);
            y = y.clamp(1.0, self.grid_height as f32 - 2.0);
        }

        true
    }

    pub fn reset(&mut self) {
        // Resize grid if dimensions changed
        let required_size = self.grid_width * self.grid_height;
        if self.grid.len() != required_size {
            self.grid = vec![None; required_size];
        } else {
            self.grid.fill(None);
        }

        // Seed with center particle
        let center_idx = self.grid_height / 2 * self.grid_width + self.grid_width / 2;
        self.grid[center_idx] = Some(0);
        self.particles_stuck = 1;
        self.max_radius = 1.0;
        self.paused = false;
    }

    pub fn reset_with_seed(&mut self, seed: &str) {
        self.grid.fill(None);

        match seed {
            "line" => {
                // Horizontal line seed
                let cy = self.grid_height / 2;
                for x in (self.grid_width / 2 - 20)..(self.grid_width / 2 + 20) {
                    self.grid[cy * self.grid_width + x] = Some(0);
                }
                self.particles_stuck = 40;
            }
            "cross" => {
                // Cross seed
                let cx = self.grid_width / 2;
                let cy = self.grid_height / 2;
                for i in 0..10 {
                    self.grid[cy * self.grid_width + (cx - i)] = Some(0);
                    self.grid[cy * self.grid_width + (cx + i)] = Some(0);
                    self.grid[(cy - i) * self.grid_width + cx] = Some(0);
                    self.grid[(cy + i) * self.grid_width + cx] = Some(0);
                }
                self.particles_stuck = 40;
            }
            "circle" => {
                // Circle seed
                let cx = self.grid_width as f32 / 2.0;
                let cy = self.grid_height as f32 / 2.0;
                let radius = 15.0;
                for angle_deg in 0..360 {
                    let angle = (angle_deg as f32).to_radians();
                    let x = (cx + radius * angle.cos()) as usize;
                    let y = (cy + radius * angle.sin()) as usize;
                    if x < self.grid_width && y < self.grid_height {
                        self.grid[y * self.grid_width + x] = Some(0);
                    }
                }
                self.particles_stuck = 360;
            }
            _ => {
                // Single center point
                let center_idx = self.grid_height / 2 * self.grid_width + self.grid_width / 2;
                self.grid[center_idx] = Some(0);
                self.particles_stuck = 1;
            }
        }

        self.max_radius = 15.0;
        self.paused = false;
    }
}

impl Simulation2D for DLA {
    fn name(&self) -> &str {
        "Diffusion-Limited Aggregation"
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

                    if let Some(age) = self.grid[idx] {
                        let color = if self.color_by_age {
                            let t = age as f32 / self.num_particles as f32;
                            self.color_scheme.map(t, true)
                        } else {
                            Color::WHITE
                        };

                        // Draw larger particles for visibility
                        colors.push(color);
                    } else {
                        colors.push(Color::BLACK);
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

        ui.heading("Diffusion-Limited Aggregation");

        ui.label(format!("Particles stuck: {} / {}",
                        self.particles_stuck, self.num_particles));

        let progress = self.particles_stuck as f32 / self.num_particles as f32;
        ui.add(egui::ProgressBar::new(progress).show_percentage());

        ui.separator();

        egui::CollapsingHeader::new("⚙ Simulation Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.num_particles, 100..=10000)
                    .text("Max Particles")).changed() {
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.stickiness, 0.1..=1.0)
                    .text("Stickiness")).changed();

                ui.label("Stickiness: chance to stick when");
                ui.label("touching another particle.");
                ui.label("Lower = more dendritic branches");
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.color_by_age, "Color by Age").changed();

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

        egui::CollapsingHeader::new("🎯 Seeds")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Point").clicked() {
                        self.reset();
                        changed = true;
                    }
                    if ui.button("Line").clicked() {
                        self.reset_with_seed("line");
                        changed = true;
                    }
                    if ui.button("Cross").clicked() {
                        self.reset_with_seed("cross");
                        changed = true;
                    }
                    if ui.button("Circle").clicked() {
                        self.reset_with_seed("circle");
                        changed = true;
                    }
                });
            });

        ui.horizontal(|ui| {
            if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                self.paused = !self.paused;
            }

            if ui.button("🔄 Reset").clicked() {
                self.reset();
                changed = true;
            }
        });

        egui::CollapsingHeader::new("📊 Info")
            .show(ui, |ui| {
                ui.label("DLA simulates particles randomly");
                ui.label("walking until they stick to the");
                ui.label("growing structure. Creates fractal");
                ui.label("snowflake-like patterns similar to");
                ui.label("crystal growth, lightning, and");
                ui.label("river deltas in nature.");
            });

        // Run simulation steps
        if !self.paused {
            for _ in 0..5 {
                if !self.step() {
                    break;
                }
            }
            changed = true;
        }

        changed
    }
}
