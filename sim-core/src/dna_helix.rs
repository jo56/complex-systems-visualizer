use crate::Simulation3D;

pub struct DNAHelix {
    pub radius: f32,
    pub helix_height: f32,
    pub twist_rate: f32,
    pub num_base_pairs: usize,
    pub animation_speed: f32,
    pub base_pair_length: f32,
    pub show_backbone: bool,
    pub show_base_pairs: bool,
    pub points_per_turn: usize,

    animation_time: f32,
    points: Vec<[f32; 3]>,
}

impl Default for DNAHelix {
    fn default() -> Self {
        Self {
            radius: 5.0,
            helix_height: 40.0,
            twist_rate: 4.0,
            num_base_pairs: 20,
            animation_speed: 1.0,
            base_pair_length: 8.0,
            show_backbone: true,
            show_base_pairs: true,
            points_per_turn: 50,
            animation_time: 0.0,
            points: Vec::new(),
        }
    }
}

impl DNAHelix {
    pub fn new() -> Self {
        let mut helix = Self::default();
        helix.regenerate();
        helix
    }

    fn regenerate(&mut self) {
        self.points.clear();

        let total_points = self.points_per_turn * self.twist_rate as usize;

        for i in 0..total_points {
            let t = i as f32 / total_points as f32;
            let angle = t * std::f32::consts::TAU * self.twist_rate;
            let y = (t - 0.5) * self.helix_height;

            // First strand
            if self.show_backbone {
                let x1 = self.radius * angle.cos();
                let z1 = self.radius * angle.sin();
                self.points.push([x1, y, z1]);
            }

            // Second strand (opposite side)
            if self.show_backbone {
                let angle2 = angle + std::f32::consts::PI;
                let x2 = self.radius * angle2.cos();
                let z2 = self.radius * angle2.sin();
                self.points.push([x2, y, z2]);
            }
        }

        // Add base pairs
        if self.show_base_pairs {
            let step = self.helix_height / self.num_base_pairs as f32;

            for i in 0..self.num_base_pairs {
                let y = (i as f32 - self.num_base_pairs as f32 / 2.0) * step;
                let angle = (i as f32 / self.num_base_pairs as f32) * std::f32::consts::TAU * self.twist_rate;

                // Create base pair connecting the two strands
                let steps = 8;
                for j in 0..steps {
                    let t = j as f32 / (steps - 1) as f32;
                    let angle1 = angle;
                    let angle2 = angle + std::f32::consts::PI;

                    let x = self.radius * (angle1.cos() * (1.0 - t) + angle2.cos() * t);
                    let z = self.radius * (angle1.sin() * (1.0 - t) + angle2.sin() * t);

                    self.points.push([x, y, z]);
                }
            }
        }
    }
}

impl Simulation3D for DNAHelix {
    fn name(&self) -> &str {
        "DNA Double Helix"
    }

    fn step(&mut self, dt: f32) {
        self.animation_time += dt * self.animation_speed;

        // Rotate the helix
        let rotation = self.animation_time * 0.5;

        // Regenerate with rotation applied
        self.points.clear();

        let total_points = self.points_per_turn * self.twist_rate as usize;

        for i in 0..total_points {
            let t = i as f32 / total_points as f32;
            let angle = t * std::f32::consts::TAU * self.twist_rate + rotation;
            let y = (t - 0.5) * self.helix_height;

            if self.show_backbone {
                let x1 = self.radius * angle.cos();
                let z1 = self.radius * angle.sin();
                self.points.push([x1, y, z1]);

                let angle2 = angle + std::f32::consts::PI;
                let x2 = self.radius * angle2.cos();
                let z2 = self.radius * angle2.sin();
                self.points.push([x2, y, z2]);
            }
        }

        if self.show_base_pairs {
            let step = self.helix_height / self.num_base_pairs as f32;

            for i in 0..self.num_base_pairs {
                let y = (i as f32 - self.num_base_pairs as f32 / 2.0) * step;
                let angle = (i as f32 / self.num_base_pairs as f32) * std::f32::consts::TAU * self.twist_rate + rotation;

                let steps = 8;
                for j in 0..steps {
                    let t = j as f32 / (steps - 1) as f32;
                    let angle1 = angle;
                    let angle2 = angle + std::f32::consts::PI;

                    let x = self.radius * (angle1.cos() * (1.0 - t) + angle2.cos() * t);
                    let z = self.radius * (angle1.sin() * (1.0 - t) + angle2.sin() * t);

                    self.points.push([x, y, z]);
                }
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.points.clone()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("DNA Double Helix");

        egui::CollapsingHeader::new("🧬 Helix Structure")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.radius, 2.0..=10.0)
                    .text("Radius")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.helix_height, 20.0..=80.0)
                    .text("Height")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.twist_rate, 1.0..=10.0)
                    .text("Twist Rate")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.num_base_pairs, 5..=50)
                    .text("Base Pairs")).changed() {
                    self.regenerate();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎨 Display Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.checkbox(&mut self.show_backbone, "Show Backbone").changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.checkbox(&mut self.show_base_pairs, "Show Base Pairs").changed() {
                    self.regenerate();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.animation_speed, 0.0..=5.0)
                    .text("Animation Speed"));

                if ui.add(egui::Slider::new(&mut self.points_per_turn, 20..=100)
                    .text("Detail Level")).changed() {
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
