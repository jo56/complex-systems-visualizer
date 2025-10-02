use crate::Simulation3D;

pub struct LorenzAttractor {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
    points: Vec<[f32; 3]>,
    current: [f32; 3],
    max_points: usize,
    pub speed: f32,
}

impl Default for LorenzAttractor {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            points: Vec::new(),
            current: [0.1, 0.0, 0.0],
            max_points: 5000,
            speed: 1.0,
        }
    }
}

impl LorenzAttractor {
    pub fn new() -> Self {
        Self::default()
    }

    fn compute_derivatives(&self, pos: [f32; 3]) -> [f32; 3] {
        let [x, y, z] = pos;
        [
            self.sigma * (y - x),
            x * (self.rho - z) - y,
            x * y - self.beta * z,
        ]
    }
}

impl Simulation3D for LorenzAttractor {
    fn name(&self) -> &str {
        "Lorenz Attractor"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.01;

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

        ui.heading("Lorenz Attractor Parameters");

        changed |= ui.add(egui::Slider::new(&mut self.sigma, 0.0..=20.0)
            .text("Sigma (σ)")).changed();

        changed |= ui.add(egui::Slider::new(&mut self.rho, 0.0..=50.0)
            .text("Rho (ρ)")).changed();

        changed |= ui.add(egui::Slider::new(&mut self.beta, 0.0..=10.0)
            .text("Beta (β)")).changed();

        ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0)
            .text("Speed"));

        ui.add(egui::Slider::new(&mut self.max_points, 100..=10000)
            .text("Trail Length"));

        if ui.button("Reset").clicked() {
            self.reset();
            changed = true;
        }

        ui.separator();
        ui.label("Interesting configurations:");
        if ui.button("Classic Butterfly").clicked() {
            self.sigma = 10.0;
            self.rho = 28.0;
            self.beta = 8.0 / 3.0;
            changed = true;
        }
        if ui.button("Chaotic").clicked() {
            self.sigma = 10.0;
            self.rho = 99.96;
            self.beta = 8.0 / 3.0;
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.points.clear();
        self.current = [0.1, 0.0, 0.0];
    }
}
