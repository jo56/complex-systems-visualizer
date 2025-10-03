use crate::{ColorScheme, Simulation3D};

pub struct AizawaAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub dt: f32,
    pub point_count: usize,
    pub trail_length: usize,
    pub color_scheme: ColorScheme,
    pub color_by_velocity: bool,
    pub color_by_height: bool,
    pub auto_rotate: bool,
    pub rotation_speed: f32,
    pub scale: f32,
    points: Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    time: f32,
}

impl Default for AizawaAttractor {
    fn default() -> Self {
        Self {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1,
            dt: 0.01,
            point_count: 5000,
            trail_length: 5000,
            color_scheme: ColorScheme::Rainbow,
            color_by_velocity: false,
            color_by_height: true,
            auto_rotate: true,
            rotation_speed: 0.3,
            scale: 50.0,
            points: Vec::new(),
            x: 0.1,
            y: 0.0,
            z: 0.0,
            time: 0.0,
        }
    }
}

impl AizawaAttractor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Simulation3D for AizawaAttractor {
    fn name(&self) -> &str {
        "Aizawa Attractor"
    }

    fn step(&mut self, dt: f32) {
        self.time += dt;

        for _ in 0..10 {
            // Aizawa equations
            let dx = (self.z - self.b) * self.x - self.d * self.y;
            let dy = self.d * self.x + (self.z - self.b) * self.y;
            let dz = self.c + self.a * self.z - (self.z * self.z * self.z) / 3.0
                   - (self.x * self.x + self.y * self.y) * (1.0 + self.e * self.z)
                   + self.f * self.z * self.x * self.x * self.x;

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

        ui.heading("Aizawa Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, 0.0..=2.0)
                    .text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.b, 0.0..=2.0)
                    .text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.c, 0.0..=2.0)
                    .text("c")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.d, 0.0..=5.0)
                    .text("d")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.e, 0.0..=1.0)
                    .text("e")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.f, 0.0..=1.0)
                    .text("f")).changed();
            });

        egui::CollapsingHeader::new("🔍 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.point_count, 100..=10000)
                    .text("Point Count")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.trail_length, 100..=10000)
                    .text("Trail Length")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.scale, 10.0..=100.0)
                    .text("Scale")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.05)
                    .text("Time Step")).changed();
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

                changed |= ui.checkbox(&mut self.color_by_height, "Color by Height").changed();
                changed |= ui.checkbox(&mut self.color_by_velocity, "Color by Velocity").changed();
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Chaotic").clicked() {
                    self.a = 0.85;
                    self.b = 0.9;
                    self.c = 0.6;
                    self.d = 4.0;
                    changed = true;
                }
                if ui.button("Stable").clicked() {
                    self.a = 1.0;
                    self.b = 0.5;
                    self.c = 0.8;
                    self.d = 3.0;
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.points.clear();
            self.x = 0.1;
            self.y = 0.0;
            self.z = 0.0;
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
