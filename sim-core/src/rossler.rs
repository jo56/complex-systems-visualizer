use crate::Simulation3D;

pub struct RosslerAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    points: Vec<[f32; 3]>,
    current: [f32; 3],
    max_points: usize,
    pub speed: f32,
}

impl Default for RosslerAttractor {
    fn default() -> Self {
        Self {
            a: 0.2,
            b: 0.2,
            c: 5.7,
            points: Vec::new(),
            current: [0.1, 0.0, 0.0],
            max_points: 5000,
            speed: 1.0,
        }
    }
}

impl RosslerAttractor {
    pub fn new() -> Self {
        let mut attractor = Self::default();
        // Pre-populate with points so it's visible immediately
        for _ in 0..500 {
            attractor.step(0.016);  // Simulate ~30 frames
        }
        attractor
    }

    fn compute_derivatives(&self, pos: [f32; 3]) -> [f32; 3] {
        let [x, y, z] = pos;
        [
            -y - z,
            x + self.a * y,
            self.b + z * (x - self.c),
        ]
    }
}

impl Simulation3D for RosslerAttractor {
    fn name(&self) -> &str {
        "Rössler Attractor"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.05;

        // Runge-Kutta 4th order integration
        let k1 = self.compute_derivatives(self.current);

        let temp = [
            self.current[0] + k1[0] * dt * 0.5,
            self.current[1] + k1[1] * dt * 0.5,
            self.current[2] + k1[2] * dt * 0.5,
        ];
        let k2 = self.compute_derivatives(temp);

        let temp = [
            self.current[0] + k2[0] * dt * 0.5,
            self.current[1] + k2[1] * dt * 0.5,
            self.current[2] + k2[2] * dt * 0.5,
        ];
        let k3 = self.compute_derivatives(temp);

        let temp = [
            self.current[0] + k3[0] * dt,
            self.current[1] + k3[1] * dt,
            self.current[2] + k3[2] * dt,
        ];
        let k4 = self.compute_derivatives(temp);

        self.current[0] += (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]) * dt / 6.0;
        self.current[1] += (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]) * dt / 6.0;
        self.current[2] += (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]) * dt / 6.0;

        self.points.push(self.current);

        if self.points.len() > self.max_points {
            self.points.remove(0);
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.points.clone()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Rössler Attractor");

        egui::CollapsingHeader::new("⚙ System Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, 0.0..=0.5)
                    .text("a")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.b, 0.0..=2.0)
                    .text("b")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.c, 0.0..=10.0)
                    .text("c")).changed();

                ui.label("Classic values: a=0.2, b=0.2, c=5.7");
            });

        egui::CollapsingHeader::new("🎬 Visualization")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0)
                    .text("Speed"));

                ui.add(egui::Slider::new(&mut self.max_points, 100..=10000)
                    .text("Trail Length"));

                if ui.button("🔄 Reset").clicked() {
                    self.reset();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("📍 Interesting Configurations")
            .show(ui, |ui| {
                if ui.button("Classic Rössler").clicked() {
                    self.a = 0.2;
                    self.b = 0.2;
                    self.c = 5.7;
                    changed = true;
                }
                if ui.button("Chaotic Spiral").clicked() {
                    self.a = 0.1;
                    self.b = 0.1;
                    self.c = 14.0;
                    changed = true;
                }
                if ui.button("Banded Chaos").clicked() {
                    self.a = 0.2;
                    self.b = 0.2;
                    self.c = 9.0;
                    changed = true;
                }
            });

        changed
    }

    fn reset(&mut self) {
        self.points.clear();
        self.current = [0.1, 0.0, 0.0];
    }
}
