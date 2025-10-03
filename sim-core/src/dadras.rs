use crate::{ColorScheme, Simulation3D};

pub struct DadrasAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub dt: f32,
    pub trail_length: usize,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    points: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
}

impl Default for DadrasAttractor {
    fn default() -> Self {
        Self {
            a: 3.0,
            b: 2.7,
            c: 1.7,
            d: 2.0,
            e: 9.0,
            dt: 0.005,
            trail_length: 5000,
            color_scheme: ColorScheme::Inferno,
            scale: 15.0,
            points: Vec::new(),
            x: 0.1,
            y: 0.1,
            z: 0.1,
        }
    }
}

impl DadrasAttractor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Simulation3D for DadrasAttractor {
    fn name(&self) -> &str {
        "Dadras Attractor"
    }

    fn step(&mut self, _dt: f32) {
        for _ in 0..10 {
            // Dadras equations
            let dx = self.y - self.a * self.x + self.b * self.y * self.z;
            let dy = self.c * self.y - self.x * self.z + self.z;
            let dz = self.d * self.x * self.y - self.e * self.z;

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

        ui.heading("Dadras Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, 1.0..=5.0)
                    .text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.b, 1.0..=5.0)
                    .text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.c, 0.5..=3.0)
                    .text("c")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.d, 0.5..=4.0)
                    .text("d")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.e, 5.0..=12.0)
                    .text("e")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.02)
                    .text("Time Step")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.trail_length, 100..=10000)
                    .text("Trail Length")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.scale, 5.0..=30.0)
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
                if ui.button("Wide Orbit").clicked() {
                    self.a = 4.0;
                    self.b = 2.0;
                    self.e = 7.0;
                    changed = true;
                }
                if ui.button("Tight Spiral").clicked() {
                    self.a = 2.5;
                    self.b = 3.5;
                    self.e = 10.0;
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
        self.y = 0.1;
        self.z = 0.1;
    }
}
