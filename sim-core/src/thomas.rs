use crate::{ColorScheme, Simulation3D};

pub struct ThomasAttractor {
    pub b: f32,
    pub dt: f32,
    pub trail_length: usize,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    points: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
}

impl Default for ThomasAttractor {
    fn default() -> Self {
        Self {
            b: 0.208186,
            dt: 0.1,
            trail_length: 5000,
            color_scheme: ColorScheme::Viridis,
            scale: 80.0,
            points: Vec::new(),
            x: 0.1,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ThomasAttractor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Simulation3D for ThomasAttractor {
    fn name(&self) -> &str {
        "Thomas Attractor"
    }

    fn step(&mut self, _dt: f32) {
        for _ in 0..10 {
            // Thomas cyclically symmetric attractor
            let dx = -self.b * self.x + self.y.sin();
            let dy = -self.b * self.y + self.z.sin();
            let dz = -self.b * self.z + self.x.sin();

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

        ui.heading("Thomas Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.b, 0.1..=0.4)
                    .text("b (Dissipation)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.dt, 0.01..=0.2)
                    .text("Time Step")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.trail_length, 100..=10000)
                    .text("Trail Length")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.scale, 20.0..=150.0)
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
                if ui.button("Dense Loops").clicked() {
                    self.b = 0.15;
                    self.dt = 0.15;
                    changed = true;
                }
                if ui.button("Sparse Flow").clicked() {
                    self.b = 0.3;
                    self.dt = 0.08;
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
        self.x = 0.1;
        self.y = 0.0;
        self.z = 0.0;
    }
}
