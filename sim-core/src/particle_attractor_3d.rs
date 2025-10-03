use crate::Simulation3D;

struct Particle {
    position: [f32; 3],
    age: f32,
    color_offset: f32,
}

pub struct ParticleAttractor3D {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
    pub num_particles: usize,
    pub particle_lifetime: f32,
    pub spawn_rate: f32,
    pub speed: f32,
    pub color_by_velocity: bool,
    pub color_by_depth: bool,
    pub particle_trails: bool,
    pub trail_length: usize,

    particles: Vec<Particle>,
    particle_trails_data: Vec<Vec<[f32; 3]>>,
    spawn_accumulator: f32,
}

impl Default for ParticleAttractor3D {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            num_particles: 50,
            particle_lifetime: 10.0,
            spawn_rate: 5.0,
            speed: 1.0,
            color_by_velocity: false,
            color_by_depth: true,
            particle_trails: true,
            trail_length: 100,
            particles: Vec::new(),
            particle_trails_data: Vec::new(),
            spawn_accumulator: 0.0,
        }
    }
}

impl ParticleAttractor3D {
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

    fn spawn_particle(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let random_state = RandomState::new();
        let mut hasher = random_state.build_hasher();
        self.particles.len().hash(&mut hasher);

        let hash_val = hasher.finish();
        let offset = (hash_val % 100) as f32 / 100.0;

        // Spawn near a random point on the attractor
        let angle = offset * std::f32::consts::TAU;
        let radius = 5.0;
        let position = [
            radius * angle.cos(),
            radius * angle.sin(),
            20.0,
        ];

        self.particles.push(Particle {
            position,
            age: 0.0,
            color_offset: offset,
        });

        if self.particle_trails {
            self.particle_trails_data.push(Vec::new());
        }
    }
}

impl Simulation3D for ParticleAttractor3D {
    fn name(&self) -> &str {
        "Particle Lorenz Attractor"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.01;

        // Spawn new particles
        self.spawn_accumulator += dt * self.spawn_rate * 60.0;
        while self.spawn_accumulator >= 1.0 && self.particles.len() < self.num_particles {
            self.spawn_particle();
            self.spawn_accumulator -= 1.0;
        }

        // Update particles
        let mut i = 0;
        while i < self.particles.len() {
            self.particles[i].age += dt;

            // Remove old particles
            if self.particles[i].age > self.particle_lifetime {
                self.particles.remove(i);
                if self.particle_trails && i < self.particle_trails_data.len() {
                    self.particle_trails_data.remove(i);
                }
                continue;
            }

            // Get current position
            let pos = self.particles[i].position;

            // Runge-Kutta 4th order integration
            let k1 = self.compute_derivatives(pos);

            let temp = [
                pos[0] + k1[0] * dt * 0.5,
                pos[1] + k1[1] * dt * 0.5,
                pos[2] + k1[2] * dt * 0.5,
            ];
            let k2 = self.compute_derivatives(temp);

            let temp = [
                pos[0] + k2[0] * dt * 0.5,
                pos[1] + k2[1] * dt * 0.5,
                pos[2] + k2[2] * dt * 0.5,
            ];
            let k3 = self.compute_derivatives(temp);

            let temp = [
                pos[0] + k3[0] * dt,
                pos[1] + k3[1] * dt,
                pos[2] + k3[2] * dt,
            ];
            let k4 = self.compute_derivatives(temp);

            self.particles[i].position[0] += (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]) * dt / 6.0;
            self.particles[i].position[1] += (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]) * dt / 6.0;
            self.particles[i].position[2] += (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]) * dt / 6.0;

            // Update trail
            if self.particle_trails && i < self.particle_trails_data.len() {
                self.particle_trails_data[i].push(self.particles[i].position);
                if self.particle_trails_data[i].len() > self.trail_length {
                    self.particle_trails_data[i].remove(0);
                }
            }

            i += 1;
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        // Add all particle positions
        for particle in &self.particles {
            points.push(particle.position);
        }

        // Add trail points
        if self.particle_trails {
            for trail in &self.particle_trails_data {
                points.extend_from_slice(trail);
            }
        }

        points
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Particle Lorenz Attractor");

        ui.label(format!("Active particles: {}", self.particles.len()));

        egui::CollapsingHeader::new("🎨 Particle Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.num_particles, 10..=200)
                    .text("Max Particles")).changed() {
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.spawn_rate, 0.1..=20.0)
                    .text("Spawn Rate"));

                changed |= ui.add(egui::Slider::new(&mut self.particle_lifetime, 1.0..=30.0)
                    .text("Particle Lifetime")).changed();

                changed |= ui.checkbox(&mut self.particle_trails, "Show Trails").changed();

                if self.particle_trails {
                    changed |= ui.add(egui::Slider::new(&mut self.trail_length, 10..=200)
                        .text("Trail Length")).changed();
                }
            });

        egui::CollapsingHeader::new("⚙ Attractor Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.sigma, 0.0..=20.0)
                    .text("Sigma (σ)")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.rho, 0.0..=50.0)
                    .text("Rho (ρ)")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.beta, 0.0..=10.0)
                    .text("Beta (β)")).changed();

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0)
                    .text("Speed"));
            });

        egui::CollapsingHeader::new("🌈 Color Settings")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.color_by_velocity, "Color by Velocity").changed();
                changed |= ui.checkbox(&mut self.color_by_depth, "Color by Depth").changed();
            });

        if ui.button("🔄 Reset").clicked() {
            self.reset();
            changed = true;
        }

        ui.separator();
        ui.label("Presets:");
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
        self.particles.clear();
        self.particle_trails_data.clear();
        self.spawn_accumulator = 0.0;
    }
}
