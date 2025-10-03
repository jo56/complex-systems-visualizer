use crate::Simulation3D;
use rand::Rng;

#[derive(Clone)]
struct TreeBranch {
    start: [f32; 3],
    end: [f32; 3],
    thickness: f32,
}

pub struct FractalTree3D {
    pub branch_angle: f32,
    pub branch_factor: f32,
    pub length_decay: f32,
    pub max_depth: usize,
    pub twist_angle: f32,
    pub randomness: f32,
    pub growth_speed: f32,
    pub speed: f32,
    pub animated_growth: bool,
    branches: Vec<TreeBranch>,
    growth_progress: f32,
    target_branches: Vec<TreeBranch>,
}

impl Default for FractalTree3D {
    fn default() -> Self {
        Self {
            branch_angle: 25.0,
            branch_factor: 3.0,
            length_decay: 0.7,
            max_depth: 8,
            twist_angle: 60.0,
            randomness: 0.15,
            growth_speed: 0.5,
            speed: 1.0,
            animated_growth: true,
            branches: Vec::new(),
            growth_progress: 0.0,
            target_branches: Vec::new(),
        }
    }
}

impl FractalTree3D {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.generate_tree();
        sim
    }

    fn generate_tree(&mut self) {
        self.target_branches.clear();
        self.growth_progress = 0.0;

        let trunk_height = 20.0;
        let start = [0.0, -30.0, 0.0];
        let end = [0.0, -30.0 + trunk_height, 0.0];

        self.target_branches.push(TreeBranch {
            start,
            end,
            thickness: 2.0,
        });

        self.grow_recursive(end, [0.0, 1.0, 0.0], trunk_height, 1.5, 1);

        if !self.animated_growth {
            self.branches = self.target_branches.clone();
            self.growth_progress = 1.0;
        } else {
            self.branches.clear();
        }
    }

    fn grow_recursive(
        &mut self,
        start: [f32; 3],
        direction: [f32; 3],
        length: f32,
        thickness: f32,
        depth: usize,
    ) {
        if depth >= self.max_depth || length < 0.5 {
            return;
        }

        let mut rng = rand::thread_rng();
        let branch_count = if depth < 3 { 3 } else { 2 }; // More branches at top

        for i in 0..branch_count {
            let angle_deg = self.branch_angle * (1.0 + rng.gen_range(-self.randomness..self.randomness));
            let angle_rad = angle_deg.to_radians();

            let twist = (i as f32 * 360.0 / branch_count as f32 + self.twist_angle).to_radians();

            // Create new direction by rotating around Y axis and tilting outward
            let dir_xz_len = (direction[0] * direction[0] + direction[2] * direction[2]).sqrt();

            let new_dir = if dir_xz_len > 0.01 {
                // Normalize XZ component
                let base_x = direction[0] / dir_xz_len;
                let base_z = direction[2] / dir_xz_len;

                // Rotate around Y axis for twist
                let twisted_x = base_x * twist.cos() - base_z * twist.sin();
                let twisted_z = base_x * twist.sin() + base_z * twist.cos();

                // Apply tilt (reduce Y component, increase XZ)
                let tilt_factor = angle_rad.sin();
                let y_component = direction[1] * angle_rad.cos();

                [
                    twisted_x * tilt_factor,
                    y_component,
                    twisted_z * tilt_factor,
                ]
            } else {
                // Starting from vertical, branch outward
                [
                    twist.cos() * angle_rad.sin(),
                    angle_rad.cos(),
                    twist.sin() * angle_rad.sin(),
                ]
            };

            // Normalize
            let mag = (new_dir[0] * new_dir[0] + new_dir[1] * new_dir[1] + new_dir[2] * new_dir[2]).sqrt();
            let normalized_dir = [new_dir[0] / mag, new_dir[1] / mag, new_dir[2] / mag];

            let new_length = length * self.length_decay * (1.0 + rng.gen_range(-self.randomness..self.randomness));
            let new_thickness = thickness * 0.7;

            let end = [
                start[0] + normalized_dir[0] * new_length,
                start[1] + normalized_dir[1] * new_length,
                start[2] + normalized_dir[2] * new_length,
            ];

            self.target_branches.push(TreeBranch {
                start,
                end,
                thickness: new_thickness,
            });

            self.grow_recursive(end, normalized_dir, new_length, new_thickness, depth + 1);
        }
    }
}

impl Simulation3D for FractalTree3D {
    fn name(&self) -> &str {
        "Fractal Tree 3D"
    }

    fn step(&mut self, dt: f32) {
        if !self.animated_growth {
            return;
        }

        let dt = dt * self.speed;

        if self.growth_progress < 1.0 {
            self.growth_progress += dt * self.growth_speed;
            self.growth_progress = self.growth_progress.min(1.0);

            // Update visible branches based on growth
            let target_count = (self.target_branches.len() as f32 * self.growth_progress) as usize;
            if self.branches.len() < target_count && target_count <= self.target_branches.len() {
                self.branches = self.target_branches[0..target_count].to_vec();
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        for branch in &self.branches {
            // Number of points based on thickness
            let point_count = (branch.thickness * 3.0).max(1.0) as usize;

            for _ in 0..point_count {
                // Add points along the branch
                let steps = 5;
                for i in 0..=steps {
                    let t = i as f32 / steps as f32;
                    points.push([
                        branch.start[0] * (1.0 - t) + branch.end[0] * t,
                        branch.start[1] * (1.0 - t) + branch.end[1] * t,
                        branch.start[2] * (1.0 - t) + branch.end[2] * t,
                    ]);
                }
            }
        }

        points
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Fractal Tree 3D");

        ui.label(format!("Branches: {} / {}", self.branches.len(), self.target_branches.len()));
        if self.animated_growth {
            ui.label(format!("Growth: {:.0}%", self.growth_progress * 100.0));
        }

        egui::CollapsingHeader::new("🌳 Tree Structure")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.max_depth, 4..=12)
                    .text("Max Depth")).changed() {
                    self.generate_tree();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.branch_factor, 2.0..=5.0)
                    .text("Branch Factor")).changed() {
                    self.generate_tree();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.length_decay, 0.5..=0.9)
                    .text("Length Decay")).changed() {
                    self.generate_tree();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("📐 Branch Angles")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.branch_angle, 10.0..=50.0)
                    .text("Branch Angle")).changed() {
                    self.generate_tree();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.twist_angle, 0.0..=180.0)
                    .text("Twist Angle")).changed() {
                    self.generate_tree();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.randomness, 0.0..=0.5)
                    .text("Randomness")).changed() {
                    self.generate_tree();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎨 Animation")
            .default_open(true)
            .show(ui, |ui| {
                if ui.checkbox(&mut self.animated_growth, "Animated Growth").changed() {
                    if self.animated_growth {
                        self.growth_progress = 0.0;
                        self.branches.clear();
                    } else {
                        self.branches = self.target_branches.clone();
                        self.growth_progress = 1.0;
                    }
                    changed = true;
                }

                if self.animated_growth {
                    ui.add(egui::Slider::new(&mut self.growth_speed, 0.1..=2.0)
                        .text("Growth Speed"));

                    ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Oak Tree").clicked() {
                    self.branch_angle = 30.0;
                    self.twist_angle = 60.0;
                    self.length_decay = 0.75;
                    self.max_depth = 7;
                    self.randomness = 0.2;
                    self.generate_tree();
                    changed = true;
                }
                if ui.button("Pine Tree").clicked() {
                    self.branch_angle = 20.0;
                    self.twist_angle = 45.0;
                    self.length_decay = 0.65;
                    self.max_depth = 9;
                    self.randomness = 0.1;
                    self.generate_tree();
                    changed = true;
                }
                if ui.button("Willow Tree").clicked() {
                    self.branch_angle = 45.0;
                    self.twist_angle = 90.0;
                    self.length_decay = 0.8;
                    self.max_depth = 6;
                    self.randomness = 0.3;
                    self.generate_tree();
                    changed = true;
                }
                if ui.button("Alien Tree").clicked() {
                    self.branch_angle = 40.0;
                    self.twist_angle = 120.0;
                    self.length_decay = 0.7;
                    self.max_depth = 8;
                    self.randomness = 0.4;
                    self.generate_tree();
                    changed = true;
                }
            });

        if ui.button("🔄 Regenerate").clicked() {
            self.generate_tree();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.generate_tree();
    }
}
