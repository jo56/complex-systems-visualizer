use crate::Simulation3D;
use rand::Rng;

struct Magnet {
    position: [f32; 3],
    strength: f32,
    polarity: f32, // 1.0 for north, -1.0 for south
}

struct FieldParticle {
    position: [f32; 3],
    velocity: [f32; 3],
    trail: Vec<[f32; 3]>,
}

pub struct MagneticField {
    pub magnet_count: usize,
    pub particle_count: usize,
    pub field_strength: f32,
    pub particle_speed: f32,
    pub trail_length: usize,
    pub show_trails: bool,
    pub damping: f32,
    pub speed: f32,
    pub spawn_radius: f32,
    pub magnet_strength: f32,
    magnets: Vec<Magnet>,
    particles: Vec<FieldParticle>,
}

impl Default for MagneticField {
    fn default() -> Self {
        Self {
            magnet_count: 2,
            particle_count: 200,
            field_strength: 50.0,
            particle_speed: 2.0,
            trail_length: 100,
            show_trails: true,
            damping: 0.98,
            speed: 1.0,
            spawn_radius: 30.0,
            magnet_strength: 100.0,
            magnets: Vec::new(),
            particles: Vec::new(),
        }
    }
}

impl MagneticField {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_simulation();
        sim
    }

    fn init_simulation(&mut self) {
        self.init_magnets();
        self.init_particles();
    }

    fn init_magnets(&mut self) {
        self.magnets.clear();

        match self.magnet_count {
            1 => {
                // Single monopole (impossible in reality, but interesting visually)
                self.magnets.push(Magnet {
                    position: [0.0, 0.0, 0.0],
                    strength: self.magnet_strength,
                    polarity: 1.0,
                });
            }
            2 => {
                // Dipole - classic bar magnet
                self.magnets.push(Magnet {
                    position: [-10.0, 0.0, 0.0],
                    strength: self.magnet_strength,
                    polarity: 1.0,
                });
                self.magnets.push(Magnet {
                    position: [10.0, 0.0, 0.0],
                    strength: self.magnet_strength,
                    polarity: -1.0,
                });
            }
            3 => {
                // Triangle arrangement
                let angles = [0.0, 2.0 * std::f32::consts::PI / 3.0, 4.0 * std::f32::consts::PI / 3.0];
                for (i, &angle) in angles.iter().enumerate() {
                    self.magnets.push(Magnet {
                        position: [15.0 * angle.cos(), 0.0, 15.0 * angle.sin()],
                        strength: self.magnet_strength,
                        polarity: if i % 2 == 0 { 1.0 } else { -1.0 },
                    });
                }
            }
            _ => {
                // Quadrupole
                self.magnets.push(Magnet {
                    position: [-10.0, 0.0, -10.0],
                    strength: self.magnet_strength,
                    polarity: 1.0,
                });
                self.magnets.push(Magnet {
                    position: [10.0, 0.0, -10.0],
                    strength: self.magnet_strength,
                    polarity: -1.0,
                });
                self.magnets.push(Magnet {
                    position: [-10.0, 0.0, 10.0],
                    strength: self.magnet_strength,
                    polarity: -1.0,
                });
                self.magnets.push(Magnet {
                    position: [10.0, 0.0, 10.0],
                    strength: self.magnet_strength,
                    polarity: 1.0,
                });
            }
        }
    }

    fn init_particles(&mut self) {
        self.particles.clear();
        let mut rng = rand::thread_rng();

        for _ in 0..self.particle_count {
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            let radius = rng.gen_range(5.0..self.spawn_radius);

            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.sin() * theta.sin();
            let z = radius * phi.cos();

            self.particles.push(FieldParticle {
                position: [x, y, z],
                velocity: [0.0, 0.0, 0.0],
                trail: Vec::new(),
            });
        }
    }

    fn compute_field(&self, position: [f32; 3]) -> [f32; 3] {
        let mut field = [0.0, 0.0, 0.0];

        for magnet in &self.magnets {
            let dx = position[0] - magnet.position[0];
            let dy = position[1] - magnet.position[1];
            let dz = position[2] - magnet.position[2];

            let dist_sq = dx * dx + dy * dy + dz * dz + 0.1; // Softening
            let dist = dist_sq.sqrt();

            let field_mag = magnet.strength * magnet.polarity / dist_sq;

            field[0] += field_mag * dx / dist;
            field[1] += field_mag * dy / dist;
            field[2] += field_mag * dz / dist;
        }

        // Limit field strength
        let mag = (field[0] * field[0] + field[1] * field[1] + field[2] * field[2]).sqrt();
        if mag > self.field_strength {
            field[0] = field[0] / mag * self.field_strength;
            field[1] = field[1] / mag * self.field_strength;
            field[2] = field[2] / mag * self.field_strength;
        }

        field
    }
}

impl Simulation3D for MagneticField {
    fn name(&self) -> &str {
        "Magnetic Field Lines"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.05;

        // Compute fields for all particles first
        let fields: Vec<[f32; 3]> = self.particles.iter()
            .map(|p| self.compute_field(p.position))
            .collect();

        for (i, particle) in self.particles.iter_mut().enumerate() {
            let field = fields[i];

            // Update velocity based on field
            particle.velocity[0] = field[0] * self.particle_speed;
            particle.velocity[1] = field[1] * self.particle_speed;
            particle.velocity[2] = field[2] * self.particle_speed;

            // Apply damping
            particle.velocity[0] *= self.damping;
            particle.velocity[1] *= self.damping;
            particle.velocity[2] *= self.damping;

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

            // Reset particle if it goes too far
            let dist_sq = particle.position[0] * particle.position[0] +
                         particle.position[1] * particle.position[1] +
                         particle.position[2] * particle.position[2];

            if dist_sq > self.spawn_radius * self.spawn_radius * 4.0 {
                let mut rng = rand::thread_rng();
                let theta = rng.gen_range(0.0..std::f32::consts::TAU);
                let phi = rng.gen_range(0.0..std::f32::consts::PI);
                let radius = rng.gen_range(5.0..self.spawn_radius);

                particle.position[0] = radius * phi.sin() * theta.cos();
                particle.position[1] = radius * phi.sin() * theta.sin();
                particle.position[2] = radius * phi.cos();
                particle.velocity = [0.0, 0.0, 0.0];
                particle.trail.clear();
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        // Add magnet positions
        for magnet in &self.magnets {
            points.push(magnet.position);
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

        ui.heading("Magnetic Field Lines");

        ui.label(format!("Magnets: {} | Particles: {}", self.magnets.len(), self.particles.len()));

        egui::CollapsingHeader::new("🧲 Magnet Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.magnet_count, 1..=4)
                    .text("Magnet Configuration")).changed() {
                    self.init_magnets();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.magnet_strength, 10.0..=500.0)
                    .text("Magnet Strength"));

                ui.label("Configurations:");
                ui.label("1 = Monopole, 2 = Dipole (Bar Magnet)");
                ui.label("3 = Triangle, 4 = Quadrupole");
            });

        egui::CollapsingHeader::new("✨ Particle Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.particle_count, 50..=500)
                    .text("Particle Count")).changed() {
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.particle_speed, 0.1..=10.0)
                    .text("Particle Speed"));

                ui.add(egui::Slider::new(&mut self.field_strength, 10.0..=200.0)
                    .text("Max Field Strength"));

                ui.add(egui::Slider::new(&mut self.spawn_radius, 10.0..=60.0)
                    .text("Spawn Radius"));
            });

        egui::CollapsingHeader::new("🎨 Visual")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_trails, "Show Trails").changed();

                if self.show_trails {
                    ui.add(egui::Slider::new(&mut self.trail_length, 10..=200)
                        .text("Trail Length"));
                }

                ui.add(egui::Slider::new(&mut self.damping, 0.9..=0.99)
                    .text("Damping"));

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                    .text("Speed"));
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Bar Magnet").clicked() {
                    self.magnet_count = 2;
                    self.init_magnets();
                    changed = true;
                }
                if ui.button("Earth's Field").clicked() {
                    self.magnet_count = 2;
                    self.magnet_strength = 150.0;
                    self.particle_count = 300;
                    self.init_simulation();
                    changed = true;
                }
                if ui.button("Complex Field").clicked() {
                    self.magnet_count = 4;
                    self.particle_count = 400;
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
