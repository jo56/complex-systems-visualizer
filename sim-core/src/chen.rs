use crate::{Color, ColorScheme, Simulation3D};

pub struct ChenAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub dt: f32,
    pub trail_length: usize,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    points: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
}

impl Default for ChenAttractor {
    fn default() -> Self {
        Self {
            a: 5.0,
            b: -10.0,
            c: -0.38,
            dt: 0.003,
            trail_length: 5000,
            color_scheme: ColorScheme::Magma,
            scale: 8.0,
            points: Vec::new(),
            x: 0.1,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ChenAttractor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Simulation3D for ChenAttractor {
    fn name(&self) -> &str {
        "Chen Attractor"
    }

    fn step(&mut self, _dt: f32) {
        for _ in 0..10 {
            // Chen attractor equations
            let dx = self.a * self.x - self.y * self.z;
            let dy = self.b * self.y + self.x * self.z;
            let dz = self.c * self.z + self.x * self.y / 3.0;

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

        ui.heading("Chen Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, 1.0..=10.0)
                    .text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.b, -20.0..=-5.0)
                    .text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.c, -1.0..=0.0)
                    .text("c")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.01)
                    .text("Time Step")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.trail_length, 100..=10000)
                    .text("Trail Length")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.scale, 3.0..=20.0)
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
                if ui.button("Butterfly Wings").clicked() {
                    self.a = 7.0;
                    self.b = -12.0;
                    self.c = -0.5;
                    changed = true;
                }
                if ui.button("Twisted Ribbon").clicked() {
                    self.a = 3.0;
                    self.b = -8.0;
                    self.c = -0.2;
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
