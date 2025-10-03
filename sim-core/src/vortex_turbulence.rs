use crate::Simulation3D;
use rand::Rng;

struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    trail: Vec<[f32; 3]>,
    life: f32,
}

pub struct VortexTurbulence {
    pub particle_count: usize,
    pub vortex_strength: f32,
    pub turbulence: f32,
    pub flow_speed: f32,
    pub trail_length: usize,
    pub show_trails: bool,
    pub vortex_count: usize,
    pub speed: f32,
    pub spawn_rate: f32,
    pub particle_life: f32,
    particles: Vec<Particle>,
    vortices: Vec<[f32; 3]>,
}

impl Default for VortexTurbulence {
    fn default() -> Self {
        Self {
            particle_count: 300,
            vortex_strength: 20.0,
            turbulence: 5.0,
            flow_speed: 3.0,
            trail_length: 50,
            show_trails: true,
            vortex_count: 3,
            speed: 1.0,
            spawn_rate: 0.3,
            particle_life: 10.0,
            particles: Vec::new(),
            vortices: Vec::new(),
        }
    }
}

impl VortexTurbulence {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_simulation();
        sim
    }

    fn init_simulation(&mut self) {
        self.init_vortices();
        self.init_particles();
    }

    fn init_vortices(&mut self) {
        self.vortices.clear();

        match self.vortex_count {
            1 => {
                // Single central vortex
                self.vortices.push([0.0, 0.0, 0.0]);
            }
            2 => {
                // Twin vortices
                self.vortices.push([-15.0, 0.0, 0.0]);
                self.vortices.push([15.0, 0.0, 0.0]);
            }
            3 => {
                // Triple vortex ring
                let angles = [0.0, 2.0 * std::f32::consts::PI / 3.0, 4.0 * std::f32::consts::PI / 3.0];
                for &angle in &angles {
                    self.vortices.push([
                        20.0 * angle.cos(),
                        0.0,
                        20.0 * angle.sin(),
                    ]);
                }
            }
            _ => {
                // Four vortex corners
                self.vortices.push([15.0, 15.0, 0.0]);
                self.vortices.push([-15.0, 15.0, 0.0]);
                self.vortices.push([15.0, -15.0, 0.0]);
                self.vortices.push([-15.0, -15.0, 0.0]);
            }
        }
    }

    fn init_particles(&mut self) {
        self.particles.clear();
        let mut rng = rand::thread_rng();

        for _ in 0..self.particle_count {
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            let radius = rng.gen_range(30.0..50.0);

            self.particles.push(Particle {
                position: [
                    radius * theta.cos(),
                    rng.gen_range(-20.0..20.0),
                    radius * theta.sin(),
                ],
                velocity: [0.0, 0.0, 0.0],
                trail: Vec::new(),
                life: rng.gen_range(0.0..self.particle_life),
            });
        }
    }

    fn compute_vortex_force(&self, pos: [f32; 3]) -> [f32; 3] {
        let mut force = [0.0, 0.0, 0.0];

        for vortex in &self.vortices {
            let dx = pos[0] - vortex[0];
            let dy = pos[1] - vortex[1];
            let dz = pos[2] - vortex[2];

            let dist_sq = dx * dx + dy * dy + dz * dz + 1.0;
            let dist = dist_sq.sqrt();

            // Tangential force (swirling)
            let tangent = [-dz, 0.0, dx]; // Perpendicular to radial direction
            let tangent_mag = (tangent[0] * tangent[0] + tangent[2] * tangent[2]).sqrt();

            if tangent_mag > 0.01 {
                let vortex_mag = self.vortex_strength / dist_sq;
                force[0] += tangent[0] / tangent_mag * vortex_mag;
                force[1] += tangent[1] / tangent_mag * vortex_mag;
                force[2] += tangent[2] / tangent_mag * vortex_mag;
            }

            // Radial attraction
            let attract_mag = self.vortex_strength * 0.3 / dist_sq;
            force[0] -= dx / dist * attract_mag;
            force[1] -= dy / dist * attract_mag;
            force[2] -= dz / dist * attract_mag;

            // Upward lift near center
            let lift = self.vortex_strength * 0.5 / (dist + 5.0);
            force[1] += lift;
        }

        force
    }

    fn spawn_particle(&mut self) {
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(30.0..50.0);

        self.particles.push(Particle {
            position: [
                radius * theta.cos(),
                rng.gen_range(-20.0..20.0),
                radius * theta.sin(),
            ],
            velocity: [0.0, 0.0, 0.0],
            trail: Vec::new(),
            life: self.particle_life,
        });
    }
}

impl Simulation3D for VortexTurbulence {
    fn name(&self) -> &str {
        "Vortex Turbulence"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.02;
        let mut rng = rand::thread_rng();

        // Compute forces for all particles
        let forces: Vec<[f32; 3]> = self.particles.iter()
            .map(|p| self.compute_vortex_force(p.position))
            .collect();

        // Update particles
        for (i, particle) in self.particles.iter_mut().enumerate() {
            let force = forces[i];

            // Add turbulence
            let turb_x = rng.gen_range(-self.turbulence..self.turbulence);
            let turb_y = rng.gen_range(-self.turbulence..self.turbulence);
            let turb_z = rng.gen_range(-self.turbulence..self.turbulence);

            // Update velocity
            particle.velocity[0] = force[0] * self.flow_speed + turb_x;
            particle.velocity[1] = force[1] * self.flow_speed + turb_y;
            particle.velocity[2] = force[2] * self.flow_speed + turb_z;

            // Limit velocity
            let vel_mag = (particle.velocity[0] * particle.velocity[0] +
                          particle.velocity[1] * particle.velocity[1] +
                          particle.velocity[2] * particle.velocity[2]).sqrt();

            if vel_mag > 30.0 {
                let scale = 30.0 / vel_mag;
                particle.velocity[0] *= scale;
                particle.velocity[1] *= scale;
                particle.velocity[2] *= scale;
            }

            // Update position
            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;
            particle.position[2] += particle.velocity[2] * dt;

            // Update trail
            if self.show_trails {
                particle.trail.push(particle.position);
                if particle.trail.len() > self.trail_length {
                    particle.trail.remove(0);
                }
            }

            // Update life
            particle.life -= dt;
        }

        // Remove dead particles
        self.particles.retain(|p| p.life > 0.0);

        // Spawn new particles
        while self.particles.len() < self.particle_count {
            if rng.gen_bool(self.spawn_rate as f64) {
                self.spawn_particle();
            } else {
                break;
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        // Add vortex centers (larger representation)
        for vortex in &self.vortices {
            for _ in 0..5 {
                points.push(*vortex);
            }
        }

        // Add particle positions
        for particle in &self.particles {
            points.push(particle.position);

            // Add trail points
            if self.show_trails {
                points.extend_from_slice(&particle.trail);
            }
        }

        points
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Vortex Turbulence");

        ui.label(format!("Vortices: {} | Particles: {}", self.vortices.len(), self.particles.len()));

        egui::CollapsingHeader::new("🌀 Vortex Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.vortex_count, 1..=4)
                    .text("Vortex Count")).changed() {
                    self.init_vortices();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.vortex_strength, 5.0..=50.0)
                    .text("Vortex Strength"));

                ui.label("Configurations:");
                ui.label("1 = Single, 2 = Twin, 3 = Ring, 4 = Corners");
            });

        egui::CollapsingHeader::new("✨ Particle Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.particle_count, 100..=1000)
                    .text("Particle Count")).changed() {
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.flow_speed, 0.5..=10.0)
                    .text("Flow Speed"));

                ui.add(egui::Slider::new(&mut self.turbulence, 0.0..=15.0)
                    .text("Turbulence"));

                ui.add(egui::Slider::new(&mut self.particle_life, 2.0..=20.0)
                    .text("Particle Life"));

                ui.add(egui::Slider::new(&mut self.spawn_rate, 0.1..=1.0)
                    .text("Spawn Rate"));
            });

        egui::CollapsingHeader::new("🎨 Visual")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_trails, "Show Trails").changed();

                if self.show_trails {
                    ui.add(egui::Slider::new(&mut self.trail_length, 10..=100)
                        .text("Trail Length"));
                }

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                    .text("Speed"));
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Tornado").clicked() {
                    self.vortex_count = 1;
                    self.vortex_strength = 35.0;
                    self.turbulence = 8.0;
                    self.particle_count = 500;
                    self.init_simulation();
                    changed = true;
                }
                if ui.button("Twin Cyclones").clicked() {
                    self.vortex_count = 2;
                    self.vortex_strength = 25.0;
                    self.turbulence = 5.0;
                    self.particle_count = 400;
                    self.init_simulation();
                    changed = true;
                }
                if ui.button("Chaotic Storm").clicked() {
                    self.vortex_count = 4;
                    self.vortex_strength = 20.0;
                    self.turbulence = 12.0;
                    self.particle_count = 600;
                    self.init_simulation();
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.init_simulation();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.init_simulation();
    }
}
