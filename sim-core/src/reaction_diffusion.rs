use crate::{Color, ColorScheme, Simulation2D};

pub struct ReactionDiffusion {
    pub feed_rate: f32,
    pub kill_rate: f32,
    pub diffusion_a: f32,
    pub diffusion_b: f32,
    pub color_scheme: ColorScheme,
    pub resolution: usize,
    pub show_grid: bool,
    grid_a: Vec<f32>,
    grid_b: Vec<f32>,
    next_a: Vec<f32>,
    next_b: Vec<f32>,
    width: usize,
    height: usize,
}

impl Default for ReactionDiffusion {
    fn default() -> Self {
        let width = 128;
        let height = 128;
        let size = width * height;

        let mut rd = Self {
            feed_rate: 0.055,
            kill_rate: 0.062,
            diffusion_a: 1.0,
            diffusion_b: 0.5,
            color_scheme: ColorScheme::Viridis,
            resolution: 128,
            show_grid: false,
            grid_a: vec![1.0; size],
            grid_b: vec![0.0; size],
            next_a: vec![0.0; size],
            next_b: vec![0.0; size],
            width,
            height,
        };

        // Seed with some B in the center
        for y in (height / 2 - 5)..(height / 2 + 5) {
            for x in (width / 2 - 5)..(width / 2 + 5) {
                rd.grid_b[y * width + x] = 1.0;
            }
        }

        rd
    }
}

impl ReactionDiffusion {
    pub fn new() -> Self {
        Self::default()
    }

    fn laplacian(&self, grid: &[f32], x: usize, y: usize) -> f32 {
        let mut sum = 0.0;

        let left = if x > 0 { x - 1 } else { self.width - 1 };
        let right = if x < self.width - 1 { x + 1 } else { 0 };
        let up = if y > 0 { y - 1 } else { self.height - 1 };
        let down = if y < self.height - 1 { y + 1 } else { 0 };

        sum += grid[y * self.width + left];
        sum += grid[y * self.width + right];
        sum += grid[up * self.width + x];
        sum += grid[down * self.width + x];
        sum += grid[up * self.width + left] * 0.05;
        sum += grid[up * self.width + right] * 0.05;
        sum += grid[down * self.width + left] * 0.05;
        sum += grid[down * self.width + right] * 0.05;

        sum /= 4.2;
        sum - grid[y * self.width + x]
    }

    fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let a = self.grid_a[idx];
                let b = self.grid_b[idx];

                let lap_a = self.laplacian(&self.grid_a, x, y);
                let lap_b = self.laplacian(&self.grid_b, x, y);

                let reaction = a * b * b;

                self.next_a[idx] = a + (self.diffusion_a * lap_a - reaction + self.feed_rate * (1.0 - a));
                self.next_b[idx] = b + (self.diffusion_b * lap_b + reaction - (self.kill_rate + self.feed_rate) * b);

                self.next_a[idx] = self.next_a[idx].clamp(0.0, 1.0);
                self.next_b[idx] = self.next_b[idx].clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.grid_a, &mut self.next_a);
        std::mem::swap(&mut self.grid_b, &mut self.next_b);
    }
}

impl Simulation2D for ReactionDiffusion {
    fn name(&self) -> &str {
        "Reaction-Diffusion"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let cell_width = width / self.width;
        let cell_height = height / self.height;

        for gy in 0..self.height {
            for gx in 0..self.width {
                let idx = gy * self.width + gx;
                let value = self.grid_b[idx];

                let color = self.color_scheme.map(value, true);

                // Fill cell
                for py in 0..cell_height {
                    for px in 0..cell_width {
                        let x = gx * cell_width + px;
                        let y = gy * cell_height + py;

                        if x < width && y < height {
                            pixels[y * width + x] = color;
                        }
                    }
                }
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Reaction-Diffusion");

        egui::CollapsingHeader::new("⚙ Gray-Scott Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.feed_rate, 0.01..=0.1)
                    .text("Feed Rate (F)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.kill_rate, 0.03..=0.08)
                    .text("Kill Rate (k)")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.diffusion_a, 0.5..=2.0)
                    .text("Diffusion A")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.diffusion_b, 0.1..=1.0)
                    .text("Diffusion B")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.resolution, 32..=256)
                    .text("Resolution")).changed() {
                    let old_width = self.width;
                    let old_height = self.height;

                    self.width = self.resolution;
                    self.height = self.resolution;
                    let size = self.width * self.height;

                    self.grid_a = vec![1.0; size];
                    self.grid_b = vec![0.0; size];
                    self.next_a = vec![0.0; size];
                    self.next_b = vec![0.0; size];

                    // Re-seed
                    for y in (self.height / 2 - 5)..(self.height / 2 + 5) {
                        for x in (self.width / 2 - 5)..(self.width / 2 + 5) {
                            if y < self.height && x < self.width {
                                self.grid_b[y * self.width + x] = 1.0;
                            }
                        }
                    }

                    changed = true;
                }

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

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Coral Growth").clicked() {
                    self.feed_rate = 0.055;
                    self.kill_rate = 0.062;
                    changed = true;
                }
                if ui.button("Spots").clicked() {
                    self.feed_rate = 0.035;
                    self.kill_rate = 0.065;
                    changed = true;
                }
                if ui.button("Stripes").clicked() {
                    self.feed_rate = 0.025;
                    self.kill_rate = 0.055;
                    changed = true;
                }
                if ui.button("Waves").clicked() {
                    self.feed_rate = 0.014;
                    self.kill_rate = 0.054;
                    changed = true;
                }
                if ui.button("Maze").clicked() {
                    self.feed_rate = 0.029;
                    self.kill_rate = 0.057;
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            let size = self.width * self.height;
            self.grid_a = vec![1.0; size];
            self.grid_b = vec![0.0; size];

            for y in (self.height / 2 - 5)..(self.height / 2 + 5) {
                for x in (self.width / 2 - 5)..(self.width / 2 + 5) {
                    self.grid_b[y * self.width + x] = 1.0;
                }
            }

            changed = true;
        }

        // Run simulation steps
        for _ in 0..5 {
            self.step();
        }
        changed = true;

        changed
    }
}
