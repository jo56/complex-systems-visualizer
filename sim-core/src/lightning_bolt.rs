use crate::Simulation3D;
use rand::Rng;

struct BoltSegment {
    start: [f32; 3],
    end: [f32; 3],
    energy: f32,
    age: f32,
}

struct BoltBranch {
    segments: Vec<BoltSegment>,
}

pub struct LightningBolt {
    pub branch_probability: f32,
    pub segment_length: f32,
    pub chaos: f32,
    pub strike_frequency: f32,
    pub fade_rate: f32,
    pub max_branches: usize,
    pub downward_bias: f32,
    pub speed: f32,
    pub energy_decay: f32,
    branches: Vec<BoltBranch>,
    time_since_strike: f32,
    strike_points: Vec<[f32; 3]>,
}

impl Default for LightningBolt {
    fn default() -> Self {
        Self {
            branch_probability: 0.15,
            segment_length: 3.0,
            chaos: 2.0,
            strike_frequency: 2.0,
            fade_rate: 0.5,
            max_branches: 20,
            downward_bias: 0.7,
            speed: 1.0,
            energy_decay: 0.95,
            branches: Vec::new(),
            time_since_strike: 0.0,
            strike_points: Vec::new(),
        }
    }
}

impl LightningBolt {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_strike_points();
        sim
    }

    fn init_strike_points(&mut self) {
        self.strike_points = vec![
            [0.0, 40.0, 0.0],
            [-20.0, 40.0, 10.0],
            [20.0, 40.0, -10.0],
        ];
    }

    fn create_lightning_strike(&mut self) {
        self.branches.clear();
        let mut rng = rand::thread_rng();

        // Choose random strike point
        let strike_idx = rng.gen_range(0..self.strike_points.len());
        let start_pos = self.strike_points[strike_idx];

        // Create main branch
        let mut main_branch = BoltBranch {
            segments: Vec::new(),
        };

        // Generate main bolt path
        let mut current_pos = start_pos;
        let target = [
            start_pos[0] + rng.gen_range(-10.0..10.0),
            -40.0,
            start_pos[2] + rng.gen_range(-10.0..10.0),
        ];

        while current_pos[1] > target[1] && main_branch.segments.len() < 50 {
            let direction = [
                (target[0] - current_pos[0]) * 0.1 + rng.gen_range(-self.chaos..self.chaos),
                -self.segment_length * self.downward_bias + rng.gen_range(-self.chaos..self.chaos),
                (target[2] - current_pos[2]) * 0.1 + rng.gen_range(-self.chaos..self.chaos),
            ];

            let next_pos = [
                current_pos[0] + direction[0],
                current_pos[1] + direction[1],
                current_pos[2] + direction[2],
            ];

            main_branch.segments.push(BoltSegment {
                start: current_pos,
                end: next_pos,
                energy: 1.0,
                age: 0.0,
            });

            // Chance to create branch
            if rng.gen_bool(self.branch_probability as f64) && self.branches.len() < self.max_branches {
                self.create_branch(current_pos, &mut rng);
            }

            current_pos = next_pos;
        }

        self.branches.push(main_branch);
    }

    fn create_branch(&mut self, start: [f32; 3], rng: &mut rand::rngs::ThreadRng) {
        let mut branch = BoltBranch {
            segments: Vec::new(),
        };

        let mut current_pos = start;
        let branch_length = rng.gen_range(3..15);

        // Branch direction (more sideways than main bolt)
        let branch_dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        for _ in 0..branch_length {
            let direction = [
                branch_dir * self.segment_length * 0.5 + rng.gen_range(-self.chaos..self.chaos),
                -self.segment_length * 0.3 + rng.gen_range(-self.chaos..self.chaos),
                rng.gen_range(-self.chaos..self.chaos),
            ];

            let next_pos = [
                current_pos[0] + direction[0],
                current_pos[1] + direction[1],
                current_pos[2] + direction[2],
            ];

            branch.segments.push(BoltSegment {
                start: current_pos,
                end: next_pos,
                energy: 0.7,
                age: 0.0,
            });

            current_pos = next_pos;
        }

        self.branches.push(branch);
    }
}

impl Simulation3D for LightningBolt {
    fn name(&self) -> &str {
        "Lightning Bolt"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed;

        // Update timer
        self.time_since_strike += dt;

        // Create new strike
        if self.time_since_strike >= self.strike_frequency {
            self.create_lightning_strike();
            self.time_since_strike = 0.0;
        }

        // Update existing bolts
        for branch in &mut self.branches {
            for segment in &mut branch.segments {
                segment.age += dt;
                segment.energy *= self.energy_decay.powf(dt * 10.0);
            }
        }

        // Remove faded branches
        self.branches.retain(|b| {
            b.segments.iter().any(|s| s.energy > 0.01)
        });
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        // Add strike points
        for point in &self.strike_points {
            for _ in 0..3 {
                points.push(*point);
            }
        }

        // Add bolt segments
        for branch in &self.branches {
            for segment in &branch.segments {
                if segment.energy > 0.01 {
                    // Add multiple points per segment based on energy
                    let steps = (segment.energy * 5.0).max(1.0) as usize;
                    for i in 0..=steps {
                        let t = i as f32 / steps as f32;
                        points.push([
                            segment.start[0] * (1.0 - t) + segment.end[0] * t,
                            segment.start[1] * (1.0 - t) + segment.end[1] * t,
                            segment.start[2] * (1.0 - t) + segment.end[2] * t,
                        ]);
                    }
                }
            }
        }

        points
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Lightning Bolt");

        ui.label(format!("Active Branches: {}", self.branches.len()));
        ui.label(format!("Next Strike: {:.1}s", (self.strike_frequency - self.time_since_strike).max(0.0)));

        egui::CollapsingHeader::new("⚡ Lightning Settings")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.strike_frequency, 0.5..=5.0)
                    .text("Strike Frequency (s)"));

                ui.add(egui::Slider::new(&mut self.segment_length, 1.0..=8.0)
                    .text("Segment Length"));

                ui.add(egui::Slider::new(&mut self.chaos, 0.5..=5.0)
                    .text("Chaos"));

                ui.add(egui::Slider::new(&mut self.downward_bias, 0.3..=1.0)
                    .text("Downward Bias"));
            });

        egui::CollapsingHeader::new("🌿 Branching")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.branch_probability, 0.0..=0.5)
                    .text("Branch Probability"));

                ui.add(egui::Slider::new(&mut self.max_branches, 5..=50)
                    .text("Max Branches"));
            });

        egui::CollapsingHeader::new("🎨 Visual")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.energy_decay, 0.8..=0.99)
                    .text("Energy Decay"));

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                    .text("Speed"));
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Gentle Storm").clicked() {
                    self.strike_frequency = 3.0;
                    self.branch_probability = 0.1;
                    self.chaos = 1.5;
                    changed = true;
                }
                if ui.button("Violent Storm").clicked() {
                    self.strike_frequency = 1.0;
                    self.branch_probability = 0.25;
                    self.chaos = 3.0;
                    self.max_branches = 30;
                    changed = true;
                }
                if ui.button("Forked Lightning").clicked() {
                    self.strike_frequency = 2.0;
                    self.branch_probability = 0.35;
                    self.chaos = 2.0;
                    self.max_branches = 40;
                    changed = true;
                }
            });

        if ui.button("⚡ Strike Now!").clicked() {
            self.create_lightning_strike();
            self.time_since_strike = 0.0;
        }

        if ui.button("🔄 Reset").clicked() {
            self.branches.clear();
            self.time_since_strike = 0.0;
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.branches.clear();
        self.time_since_strike = 0.0;
    }
}
