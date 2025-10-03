use crate::Simulation3D;
use rand::Rng;

struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    density: f32,
    pressure: f32,
}

pub struct FluidSPH {
    pub particle_count: usize,
    pub gravity: f32,
    pub rest_density: f32,
    pub gas_constant: f32,
    pub viscosity: f32,
    pub smoothing_radius: f32,
    pub particle_mass: f32,
    pub damping: f32,
    pub speed: f32,
    pub boundary_size: f32,
    particles: Vec<Particle>,
}

impl Default for FluidSPH {
    fn default() -> Self {
        Self {
            particle_count: 500,
            gravity: 9.8,
            rest_density: 1000.0,
            gas_constant: 2000.0,
            viscosity: 0.5,
            smoothing_radius: 2.0,
            particle_mass: 1.0,
            damping: 0.95,
            speed: 1.0,
            boundary_size: 25.0,
            particles: Vec::new(),
        }
    }
}

impl FluidSPH {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_particles();
        sim
    }

    fn init_particles(&mut self) {
        self.particles.clear();
        let mut rng = rand::thread_rng();

        let particles_per_side = (self.particle_count as f32).cbrt() as usize;
        let spacing = 1.5;

        for i in 0..particles_per_side {
            for j in 0..particles_per_side {
                for k in 0..particles_per_side {
                    if self.particles.len() >= self.particle_count {
                        break;
                    }

                    let x = -self.boundary_size / 2.0 + i as f32 * spacing + rng.gen_range(-0.2..0.2);
                    let y = self.boundary_size / 2.0 - j as f32 * spacing;
                    let z = -self.boundary_size / 2.0 + k as f32 * spacing + rng.gen_range(-0.2..0.2);

                    self.particles.push(Particle {
                        position: [x, y, z],
                        velocity: [0.0, 0.0, 0.0],
                        density: self.rest_density,
                        pressure: 0.0,
                    });
                }
            }
        }
    }

    fn smoothing_kernel(&self, r: f32) -> f32 {
        let h = self.smoothing_radius;
        if r >= h {
            return 0.0;
        }
        let volume = std::f32::consts::PI * h.powi(4) / 6.0;
        (h * h - r * r).powi(3) / volume
    }

    fn smoothing_kernel_gradient(&self, r: f32) -> f32 {
        let h = self.smoothing_radius;
        if r >= h {
            return 0.0;
        }
        let volume = std::f32::consts::PI * h.powi(4) / 6.0;
        -6.0 * (h * h - r * r).powi(2) / volume
    }

    fn compute_density_pressure(&mut self) {
        for i in 0..self.particles.len() {
            let mut density = 0.0;

            for j in 0..self.particles.len() {
                let dx = self.particles[j].position[0] - self.particles[i].position[0];
                let dy = self.particles[j].position[1] - self.particles[i].position[1];
                let dz = self.particles[j].position[2] - self.particles[i].position[2];

                let r = (dx * dx + dy * dy + dz * dz).sqrt();

                if r < self.smoothing_radius {
                    density += self.particle_mass * self.smoothing_kernel(r);
                }
            }

            self.particles[i].density = density.max(self.rest_density);
            self.particles[i].pressure = self.gas_constant * (self.particles[i].density - self.rest_density);
        }
    }

    fn compute_forces(&self) -> Vec<[f32; 3]> {
        let mut forces = vec![[0.0, 0.0, 0.0]; self.particles.len()];

        for i in 0..self.particles.len() {
            let mut pressure_force = [0.0, 0.0, 0.0];
            let mut viscosity_force = [0.0, 0.0, 0.0];

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let dx = self.particles[j].position[0] - self.particles[i].position[0];
                let dy = self.particles[j].position[1] - self.particles[i].position[1];
                let dz = self.particles[j].position[2] - self.particles[i].position[2];

                let r = (dx * dx + dy * dy + dz * dz).sqrt();

                if r < self.smoothing_radius && r > 0.0 {
                    let grad = self.smoothing_kernel_gradient(r);

                    // Pressure force
                    let pressure_term = (self.particles[i].pressure + self.particles[j].pressure) /
                                       (2.0 * self.particles[j].density);

                    pressure_force[0] -= self.particle_mass * pressure_term * grad * dx / r;
                    pressure_force[1] -= self.particle_mass * pressure_term * grad * dy / r;
                    pressure_force[2] -= self.particle_mass * pressure_term * grad * dz / r;

                    // Viscosity force
                    let kernel = self.smoothing_kernel(r);
                    let vel_diff_x = self.particles[j].velocity[0] - self.particles[i].velocity[0];
                    let vel_diff_y = self.particles[j].velocity[1] - self.particles[i].velocity[1];
                    let vel_diff_z = self.particles[j].velocity[2] - self.particles[i].velocity[2];

                    viscosity_force[0] += self.viscosity * self.particle_mass * vel_diff_x / self.particles[j].density * kernel;
                    viscosity_force[1] += self.viscosity * self.particle_mass * vel_diff_y / self.particles[j].density * kernel;
                    viscosity_force[2] += self.viscosity * self.particle_mass * vel_diff_z / self.particles[j].density * kernel;
                }
            }

            // Add gravity
            forces[i][0] = pressure_force[0] + viscosity_force[0];
            forces[i][1] = pressure_force[1] + viscosity_force[1] - self.gravity * self.particles[i].density;
            forces[i][2] = pressure_force[2] + viscosity_force[2];
        }

        forces
    }

    fn integrate(&mut self, dt: f32) {
        let forces = self.compute_forces();

        for (i, particle) in self.particles.iter_mut().enumerate() {
            let ax = forces[i][0] / particle.density;
            let ay = forces[i][1] / particle.density;
            let az = forces[i][2] / particle.density;

            particle.velocity[0] += ax * dt;
            particle.velocity[1] += ay * dt;
            particle.velocity[2] += az * dt;

            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;
            particle.position[2] += particle.velocity[2] * dt;

            // Boundary conditions
            let boundary = self.boundary_size / 2.0;

            if particle.position[0] < -boundary {
                particle.position[0] = -boundary;
                particle.velocity[0] *= -self.damping;
            } else if particle.position[0] > boundary {
                particle.position[0] = boundary;
                particle.velocity[0] *= -self.damping;
            }

            if particle.position[1] < -boundary {
                particle.position[1] = -boundary;
                particle.velocity[1] *= -self.damping;
            } else if particle.position[1] > boundary {
                particle.position[1] = boundary;
                particle.velocity[1] *= -self.damping;
            }

            if particle.position[2] < -boundary {
                particle.position[2] = -boundary;
                particle.velocity[2] *= -self.damping;
            } else if particle.position[2] > boundary {
                particle.position[2] = boundary;
                particle.velocity[2] *= -self.damping;
            }
        }
    }
}

impl Simulation3D for FluidSPH {
    fn name(&self) -> &str {
        "Fluid Simulation (SPH)"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.01;

        self.compute_density_pressure();
        self.integrate(dt);
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.particles.iter().map(|p| p.position).collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Fluid Simulation (SPH)");

        ui.label(format!("Active particles: {}", self.particles.len()));

        egui::CollapsingHeader::new("💧 Particle Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.particle_count, 50..=1000)
                    .text("Particle Count")).changed() {
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.smoothing_radius, 0.5..=5.0)
                    .text("Smoothing Radius"));

                ui.add(egui::Slider::new(&mut self.particle_mass, 0.1..=5.0)
                    .text("Particle Mass"));
            });

        egui::CollapsingHeader::new("⚙ Physics")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.gravity, 0.0..=20.0)
                    .text("Gravity"));

                ui.add(egui::Slider::new(&mut self.gas_constant, 500.0..=5000.0)
                    .text("Pressure"));

                ui.add(egui::Slider::new(&mut self.viscosity, 0.0..=2.0)
                    .text("Viscosity"));

                ui.add(egui::Slider::new(&mut self.damping, 0.5..=0.99)
                    .text("Damping"));

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                    .text("Speed"));

                ui.add(egui::Slider::new(&mut self.boundary_size, 10.0..=50.0)
                    .text("Boundary Size"));
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Water Drop").clicked() {
                    self.gravity = 15.0;
                    self.viscosity = 0.3;
                    self.gas_constant = 2000.0;
                    self.init_particles();
                    changed = true;
                }
                if ui.button("Honey").clicked() {
                    self.gravity = 10.0;
                    self.viscosity = 1.5;
                    self.gas_constant = 1500.0;
                    self.init_particles();
                    changed = true;
                }
                if ui.button("Gas Cloud").clicked() {
                    self.gravity = 2.0;
                    self.viscosity = 0.1;
                    self.gas_constant = 3000.0;
                    self.init_particles();
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.init_particles();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.init_particles();
    }
}
