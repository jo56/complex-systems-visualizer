use crate::{Color, ColorScheme, Simulation3D};

pub struct HalvorsenAttractor {
    pub a: f32,
    pub dt: f32,
    pub point_count: usize,
    pub trail_length: usize,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    points: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
}

impl Default for HalvorsenAttractor {
    fn default() -> Self {
        Self {
            a: 1.89,
            dt: 0.005,
            point_count: 5000,
            trail_length: 5000,
            color_scheme: ColorScheme::Plasma,
            scale: 20.0,
            points: Vec::new(),
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl HalvorsenAttractor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Simulation3D for HalvorsenAttractor {
    fn name(&self) -> &str {
        "Halvorsen Attractor"
    }

    fn step(&mut self, _dt: f32) {
        for _ in 0..10 {
            // Halvorsen equations
            let dx = -self.a * self.x - 4.0 * self.y - 4.0 * self.z - self.y * self.y;
            let dy = -self.a * self.y - 4.0 * self.z - 4.0 * self.x - self.z * self.z;
            let dz = -self.a * self.z - 4.0 * self.x - 4.0 * self.y - self.x * self.x;

            self.x += dx * self.dt;
            self.y += dy * self.dt;
            self.z += dz * self.dt;

            self.points.push([
                self.x * self.scale,
                self.y * self.scale,
                self.z * self.scale,
            ]);

            if self.points.len() > self.trail_length {
                self.points.remove(0);
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.points.clone()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Halvorsen Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, 0.5..=3.0)
                    .text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.02)
                    .text("Time Step")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.trail_length, 100..=10000)
                    .text("Trail Length")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.scale, 5.0..=50.0)
                    .text("Scale")).changed();
            });

        egui::CollapsingHeader::new("🎨 Color Settings")
            .show(ui, |ui| {
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
                if ui.button("Classic").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Fast Chaos").clicked() {
                    self.a = 2.5;
                    self.dt = 0.008;
                    changed = true;
                }
                if ui.button("Slow Flow").clicked() {
                    self.a = 1.2;
                    self.dt = 0.003;
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.reset();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.points.clear();
        self.x = -1.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}
