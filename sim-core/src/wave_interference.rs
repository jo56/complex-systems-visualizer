use crate::{Color, ColorScheme, Simulation2D};

pub struct WaveInterference {
    pub wave_count: usize,
    pub wavelength: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub color_scheme: ColorScheme,
    pub show_sources: bool,
    pub damping: f32,
    sources: Vec<(f32, f32)>,
    animation_time: f32,
}

impl Default for WaveInterference {
    fn default() -> Self {
        Self {
            wave_count: 3,
            wavelength: 50.0,
            amplitude: 100.0,
            speed: 2.0,
            color_scheme: ColorScheme::Ocean,
            show_sources: true,
            damping: 0.0,
            sources: vec![
                (0.3, 0.3),
                (0.7, 0.3),
                (0.5, 0.7),
            ],
            animation_time: 0.0,
        }
    }
}

impl WaveInterference {
    pub fn new() -> Self {
        Self::default()
    }

    fn draw_circle(&self, pixels: &mut [Color], width: usize, height: usize,
                   cx: f32, cy: f32, radius: f32, color: Color) {
        let r_sq = radius * radius;
        let min_x = (cx - radius).max(0.0) as usize;
        let max_x = (cx + radius).min(width as f32 - 1.0) as usize;
        let min_y = (cy - radius).max(0.0) as usize;
        let max_y = (cy + radius).min(height as f32 - 1.0) as usize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= r_sq {
                    pixels[y * width + x] = color;
                }
            }
        }
    }
}

impl Simulation2D for WaveInterference {
    fn name(&self) -> &str {
        "Wave Interference"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let active_sources = self.wave_count.min(self.sources.len());

        for py in 0..height {
            for px in 0..width {
                let x = px as f32;
                let y = py as f32;

                let mut wave_sum = 0.0;

                for i in 0..active_sources {
                    let (sx, sy) = self.sources[i];
                    let source_x = width as f32 * sx;
                    let source_y = height as f32 * sy;

                    let dx = x - source_x;
                    let dy = y - source_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    let wave_phase = (distance / self.wavelength) * std::f32::consts::TAU
                                   - self.animation_time * self.speed;

                    let damping_factor = if self.damping > 0.0 {
                        (-distance * self.damping / 100.0).exp()
                    } else {
                        1.0
                    };

                    wave_sum += wave_phase.sin() * damping_factor;
                }

                wave_sum /= active_sources as f32;
                let intensity = (wave_sum + 1.0) / 2.0;

                let color = self.color_scheme.map(intensity, true);
                pixels[py * width + px] = color;
            }
        }

        // Draw source markers
        if self.show_sources {
            for i in 0..active_sources {
                let (sx, sy) = self.sources[i];
                let x = width as f32 * sx;
                let y = height as f32 * sy;

                self.draw_circle(&mut pixels, width, height, x, y, 8.0, Color::WHITE);
                self.draw_circle(&mut pixels, width, height, x, y, 5.0, Color::RED);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Wave Interference");

        egui::CollapsingHeader::new("⚙ Wave Parameters")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.wave_count, 1..=6)
                    .text("Wave Sources")).changed() {
                    // Ensure we have enough sources
                    while self.sources.len() < self.wave_count {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        self.sources.push((rng.gen_range(0.2..0.8), rng.gen_range(0.2..0.8)));
                    }
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.wavelength, 10.0..=150.0)
                    .text("Wavelength")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.amplitude, 10.0..=200.0)
                    .text("Amplitude")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.speed, 0.1..=10.0)
                    .text("Speed")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.damping, 0.0..=2.0)
                    .text("Damping")).changed();
            });

        egui::CollapsingHeader::new("📍 Source Positions")
            .show(ui, |ui| {
                for i in 0..self.wave_count.min(self.sources.len()) {
                    ui.horizontal(|ui| {
                        ui.label(format!("Source {}", i + 1));
                        changed |= ui.add(egui::Slider::new(&mut self.sources[i].0, 0.0..=1.0)
                            .text("X")).changed();
                        changed |= ui.add(egui::Slider::new(&mut self.sources[i].1, 0.0..=1.0)
                            .text("Y")).changed();
                    });
                }
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_sources, "Show Sources").changed();

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
                if ui.button("Two Waves").clicked() {
                    self.wave_count = 2;
                    self.sources[0] = (0.3, 0.5);
                    self.sources[1] = (0.7, 0.5);
                    changed = true;
                }
                if ui.button("Three Waves").clicked() {
                    self.wave_count = 3;
                    self.sources[0] = (0.3, 0.3);
                    self.sources[1] = (0.7, 0.3);
                    self.sources[2] = (0.5, 0.7);
                    changed = true;
                }
                if ui.button("Four Corners").clicked() {
                    self.wave_count = 4;
                    self.sources[0] = (0.25, 0.25);
                    self.sources[1] = (0.75, 0.25);
                    self.sources[2] = (0.25, 0.75);
                    self.sources[3] = (0.75, 0.75);
                    changed = true;
                }
                if ui.button("Circle").clicked() {
                    self.wave_count = 6;
                    for i in 0..6 {
                        let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
                        self.sources[i] = (
                            0.5 + 0.3 * angle.cos(),
                            0.5 + 0.3 * angle.sin(),
                        );
                    }
                    changed = true;
                }
            });

        // Update animation
        let dt = ui.input(|i| i.stable_dt);
        self.animation_time += dt;
        changed = true;

        changed
    }
}
