use crate::Simulation3D;
use rand::Rng;

struct Body {
    position: [f32; 3],
    velocity: [f32; 3],
    mass: f32,
    trail: Vec<[f32; 3]>,
}

pub struct NBodyGravity {
    pub body_count: usize,
    pub gravitational_constant: f32,
    pub softening: f32,
    pub speed: f32,
    pub trail_length: usize,
    pub show_trails: bool,
    pub central_mass: f32,
    pub spawn_radius: f32,
    pub initial_velocity: f32,
    bodies: Vec<Body>,
}

impl Default for NBodyGravity {
    fn default() -> Self {
        Self {
            body_count: 100,
            gravitational_constant: 1.0,
            softening: 0.5,
            speed: 1.0,
            trail_length: 50,
            show_trails: true,
            central_mass: 100.0,
            spawn_radius: 30.0,
            initial_velocity: 2.0,
            bodies: Vec::new(),
        }
    }
}

impl NBodyGravity {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_bodies();
        sim
    }

    fn init_bodies(&mut self) {
        self.bodies.clear();
        let mut rng = rand::thread_rng();

        // Add central massive body
        self.bodies.push(Body {
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            mass: self.central_mass,
            trail: Vec::new(),
        });

        // Add orbiting bodies
        for _ in 0..self.body_count {
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            let phi = rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0);
            let radius = rng.gen_range(self.spawn_radius * 0.5..self.spawn_radius);

            let x = radius * phi.cos() * theta.cos();
            let y = radius * phi.sin();
            let z = radius * phi.cos() * theta.sin();

            // Orbital velocity perpendicular to radius
            let orbital_speed = self.initial_velocity * (self.central_mass / radius).sqrt();
            let vx = -orbital_speed * theta.sin();
            let vy = rng.gen_range(-0.5..0.5);
            let vz = orbital_speed * theta.cos();

            self.bodies.push(Body {
                position: [x, y, z],
                velocity: [vx, vy, vz],
                mass: rng.gen_range(0.1..1.0),
                trail: Vec::new(),
            });
        }
    }

    fn compute_forces(&self) -> Vec<[f32; 3]> {
        let mut forces = vec![[0.0, 0.0, 0.0]; self.bodies.len()];

        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let dx = self.bodies[j].position[0] - self.bodies[i].position[0];
                let dy = self.bodies[j].position[1] - self.bodies[i].position[1];
                let dz = self.bodies[j].position[2] - self.bodies[i].position[2];

                let dist_sq = dx * dx + dy * dy + dz * dz + self.softening * self.softening;
                let dist = dist_sq.sqrt();
                let force_mag = self.gravitational_constant * self.bodies[i].mass * self.bodies[j].mass / dist_sq;

                let fx = force_mag * dx / dist;
                let fy = force_mag * dy / dist;
                let fz = force_mag * dz / dist;

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[i][2] += fz;

                forces[j][0] -= fx;
                forces[j][1] -= fy;
                forces[j][2] -= fz;
            }
        }

        forces
    }
}

impl Simulation3D for NBodyGravity {
    fn name(&self) -> &str {
        "N-Body Gravity"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed * 0.1;

        // Compute forces
        let forces = self.compute_forces();

        // Update velocities and positions
        for (i, body) in self.bodies.iter_mut().enumerate() {
            if i == 0 {
                // Keep central body stationary
                continue;
            }

            let ax = forces[i][0] / body.mass;
            let ay = forces[i][1] / body.mass;
            let az = forces[i][2] / body.mass;

            body.velocity[0] += ax * dt;
            body.velocity[1] += ay * dt;
            body.velocity[2] += az * dt;

            body.position[0] += body.velocity[0] * dt;
            body.position[1] += body.velocity[1] * dt;
            body.position[2] += body.velocity[2] * dt;

            // Update trail
            if self.show_trails {
                body.trail.push(body.position);
                if body.trail.len() > self.trail_length {
                    body.trail.remove(0);
                }
            }
        }
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        let mut points = Vec::new();

        // Add body positions
        for body in &self.bodies {
            points.push(body.position);

            // Add trail points
            if self.show_trails {
                points.extend_from_slice(&body.trail);
            }
        }

        points
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("N-Body Gravity");

        ui.label(format!("Active bodies: {}", self.bodies.len()));

        egui::CollapsingHeader::new("🌍 Body Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.body_count, 10..=200)
                    .text("Body Count")).changed() {
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.central_mass, 10.0..=500.0)
                    .text("Central Mass")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.spawn_radius, 10.0..=80.0)
                    .text("Spawn Radius")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.initial_velocity, 0.5..=5.0)
                    .text("Initial Velocity")).changed();
            });

        egui::CollapsingHeader::new("⚙ Physics")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.gravitational_constant, 0.1..=5.0)
                    .text("Gravity Strength"));

                ui.add(egui::Slider::new(&mut self.softening, 0.1..=2.0)
                    .text("Softening"));

                ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
                    .text("Speed"));
            });

        egui::CollapsingHeader::new("✨ Visual")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_trails, "Show Trails").changed();

                if self.show_trails {
                    ui.add(egui::Slider::new(&mut self.trail_length, 10..=200)
                        .text("Trail Length"));
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Solar System").clicked() {
                    self.central_mass = 200.0;
                    self.body_count = 50;
                    self.spawn_radius = 50.0;
                    self.initial_velocity = 2.5;
                    self.init_bodies();
                    changed = true;
                }
                if ui.button("Binary Stars").clicked() {
                    self.bodies.clear();
                    self.bodies.push(Body {
                        position: [-10.0, 0.0, 0.0],
                        velocity: [0.0, 2.0, 0.0],
                        mass: 50.0,
                        trail: Vec::new(),
                    });
                    self.bodies.push(Body {
                        position: [10.0, 0.0, 0.0],
                        velocity: [0.0, -2.0, 0.0],
                        mass: 50.0,
                        trail: Vec::new(),
                    });
                    // Add some debris
                    let mut rng = rand::thread_rng();
                    for _ in 0..30 {
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        let radius = rng.gen_range(25.0..40.0);
                        self.bodies.push(Body {
                            position: [radius * angle.cos(), 0.0, radius * angle.sin()],
                            velocity: [-2.0 * angle.sin(), 0.0, 2.0 * angle.cos()],
                            mass: 0.1,
                            trail: Vec::new(),
                        });
                    }
                    changed = true;
                }
                if ui.button("Chaotic Cloud").clicked() {
                    self.bodies.clear();
                    let mut rng = rand::thread_rng();
                    for _ in 0..100 {
                        self.bodies.push(Body {
                            position: [
                                rng.gen_range(-30.0..30.0),
                                rng.gen_range(-30.0..30.0),
                                rng.gen_range(-30.0..30.0),
                            ],
                            velocity: [
                                rng.gen_range(-1.0..1.0),
                                rng.gen_range(-1.0..1.0),
                                rng.gen_range(-1.0..1.0),
                            ],
                            mass: rng.gen_range(0.5..2.0),
                            trail: Vec::new(),
                        });
                    }
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.init_bodies();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.init_bodies();
    }
}
