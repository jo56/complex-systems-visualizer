use crate::Simulation3D;

pub struct TorusKnot {
    pub p: i32,  // Number of times the knot winds around the torus longitudinally
    pub q: i32,  // Number of times it winds around meridionally
    pub major_radius: f32,
    pub minor_radius: f32,
    pub tube_radius: f32,
    pub num_points: usize,
    pub animation_speed: f32,
    pub show_tube: bool,
    pub tube_segments: usize,

    animation_time: f32,
    points: Vec<[f32; 3]>,
}

impl Default for TorusKnot {
    fn default() -> Self {
        Self {
            p: 3,
            q: 2,
            major_radius: 20.0,
            minor_radius: 10.0,
            tube_radius: 2.0,
            num_points: 500,
            animation_speed: 1.0,
            show_tube: true,
            tube_segments: 8,
            animation_time: 0.0,
            points: Vec::new(),
        }
    }
}

impl TorusKnot {
    pub fn new() -> Self {
        let mut knot = Self::default();
        knot.regenerate();
        knot
    }

    fn regenerate(&mut self) {
        self.points.clear();

        for i in 0..self.num_points {
            let t = (i as f32 / self.num_points as f32) * std::f32::consts::TAU;

            let r = self.major_radius + self.minor_radius * (self.q as f32 * t).cos();
            let x = r * (self.p as f32 * t).cos();
            let y = r * (self.p as f32 * t).sin();
            let z = self.minor_radius * (self.q as f32 * t).sin();

            self.points.push([x, y, z]);

            // Add tube cross-sections for more visual appeal
            if self.show_tube {
                for j in 0..self.tube_segments {
                    let angle = (j as f32 / self.tube_segments as f32) * std::f32::consts::TAU;

                    // Calculate tangent for proper tube orientation
                    let dt = 0.01;
                    let t_next = t + dt;
                    let r_next = self.major_radius + self.minor_radius * (self.q as f32 * t_next).cos();
                    let dx = r_next * (self.p as f32 * t_next).cos() - x;
                    let dy = r_next * (self.p as f32 * t_next).sin() - y;
                    let _dz = self.minor_radius * (self.q as f32 * t_next).sin() - z;

                    // Create a simple perpendicular vector
                    let perp_x = -dy;
                    let perp_y = dx;
                    let mag = (perp_x * perp_x + perp_y * perp_y).sqrt();

                    if mag > 0.001 {
                        let tube_x = x + (self.tube_radius * angle.cos() * perp_x / mag);
                        let tube_y = y + (self.tube_radius * angle.cos() * perp_y / mag);
                        let tube_z = z + self.tube_radius * angle.sin();

                        self.points.push([tube_x, tube_y, tube_z]);
                    }
                }
            }
        }
    }

    fn get_animated_points(&self) -> Vec<[f32; 3]> {
        let rotation = self.animation_time * 0.3;
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();

        self.points.iter().map(|&[x, y, z]| {
            // Rotate around Z axis
            let x_rot = x * cos_r - y * sin_r;
            let y_rot = x * sin_r + y * cos_r;

            [x_rot, y_rot, z]
        }).collect()
    }
}

impl Simulation3D for TorusKnot {
    fn name(&self) -> &str {
        "Torus Knot"
    }

    fn step(&mut self, dt: f32) {
        self.animation_time += dt * self.animation_speed;
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.get_animated_points()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Torus Knot");

        ui.label(format!("Current knot: ({}, {})", self.p, self.q));

        egui::CollapsingHeader::new("🌀 Knot Parameters")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.p, 1..=10)
                    .text("P (longitudinal)")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.q, 1..=10)
                    .text("Q (meridional)")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.major_radius, 10.0..=40.0)
                    .text("Major Radius")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.minor_radius, 5.0..=20.0)
                    .text("Minor Radius")).changed() {
                    self.regenerate();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.checkbox(&mut self.show_tube, "Show Tube").changed() {
                    self.regenerate();
                    changed = true;
                }

                if self.show_tube {
                    if ui.add(egui::Slider::new(&mut self.tube_radius, 0.5..=5.0)
                        .text("Tube Radius")).changed() {
                        self.regenerate();
                        changed = true;
                    }

                    if ui.add(egui::Slider::new(&mut self.tube_segments, 4..=16)
                        .text("Tube Detail")).changed() {
                        self.regenerate();
                        changed = true;
                    }
                }

                if ui.add(egui::Slider::new(&mut self.num_points, 100..=1000)
                    .text("Resolution")).changed() {
                    self.regenerate();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.animation_speed, 0.0..=5.0)
                    .text("Animation Speed"));
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Trefoil (3,2)").clicked() {
                    self.p = 3;
                    self.q = 2;
                    self.regenerate();
                    changed = true;
                }
                if ui.button("Cinquefoil (5,2)").clicked() {
                    self.p = 5;
                    self.q = 2;
                    self.regenerate();
                    changed = true;
                }
                if ui.button("Figure-8 (4,3)").clicked() {
                    self.p = 4;
                    self.q = 3;
                    self.regenerate();
                    changed = true;
                }
                if ui.button("Complex (7,3)").clicked() {
                    self.p = 7;
                    self.q = 3;
                    self.regenerate();
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            *self = Self::new();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.animation_time = 0.0;
        self.regenerate();
    }
}
