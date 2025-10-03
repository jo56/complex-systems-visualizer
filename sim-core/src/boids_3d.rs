use crate::Simulation3D;

struct Boid3D {
    position: [f32; 3],
    velocity: [f32; 3],
}

pub struct Boids3D {
    pub boid_count: usize,
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,
    pub separation_strength: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub bound_radius: f32,
    pub speed: f32,

    boids: Vec<Boid3D>,
}

impl Default for Boids3D {
    fn default() -> Self {
        Self {
            boid_count: 50,
            separation_radius: 5.0,
            alignment_radius: 10.0,
            cohesion_radius: 10.0,
            separation_strength: 1.5,
            alignment_strength: 1.0,
            cohesion_strength: 1.0,
            max_speed: 2.0,
            max_force: 0.1,
            bound_radius: 30.0,
            speed: 1.0,
            boids: Vec::new(),
        }
    }
}

impl Boids3D {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_boids();
        sim
    }

    fn init_boids(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        self.boids.clear();
        let random_state = RandomState::new();

        for i in 0..self.boid_count {
            let mut hasher = random_state.build_hasher();
            i.hash(&mut hasher);
            let hash1 = hasher.finish();

            let mut hasher = random_state.build_hasher();
            (i * 2).hash(&mut hasher);
            let hash2 = hasher.finish();

            let mut hasher = random_state.build_hasher();
            (i * 3).hash(&mut hasher);
            let hash3 = hasher.finish();

            let theta = (hash1 % 1000) as f32 / 1000.0 * std::f32::consts::TAU;
            let phi = (hash2 % 1000) as f32 / 1000.0 * std::f32::consts::PI;
            let r = (hash3 % 1000) as f32 / 1000.0 * self.bound_radius * 0.5;

            let position = [
                r * phi.sin() * theta.cos(),
                r * phi.sin() * theta.sin(),
                r * phi.cos(),
            ];

            let speed = (hash1 % 100) as f32 / 100.0 * self.max_speed;
            let vtheta = (hash2 % 1000) as f32 / 1000.0 * std::f32::consts::TAU;
            let vphi = (hash3 % 1000) as f32 / 1000.0 * std::f32::consts::PI;

            let velocity = [
                speed * vphi.sin() * vtheta.cos(),
                speed * vphi.sin() * vtheta.sin(),
                speed * vphi.cos(),
            ];

            self.boids.push(Boid3D { position, velocity });
        }
    }

    fn update_boids(&mut self, dt: f32) {
        let boid_data: Vec<([f32; 3], [f32; 3])> = self.boids
            .iter()
            .map(|b| (b.position, b.velocity))
            .collect();

        for (i, boid) in self.boids.iter_mut().enumerate() {
            let mut sep = [0.0, 0.0, 0.0];
            let mut sep_count = 0;

            let mut align = [0.0, 0.0, 0.0];
            let mut align_count = 0;

            let mut coh = [0.0, 0.0, 0.0];
            let mut coh_count = 0;

            // Calculate flocking forces
            for (j, &(other_pos, other_vel)) in boid_data.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dx = other_pos[0] - boid.position[0];
                let dy = other_pos[1] - boid.position[1];
                let dz = other_pos[2] - boid.position[2];
                let dist_sq = dx * dx + dy * dy + dz * dz;

                // Separation
                if dist_sq < self.separation_radius * self.separation_radius && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    sep[0] -= dx / dist;
                    sep[1] -= dy / dist;
                    sep[2] -= dz / dist;
                    sep_count += 1;
                }

                // Alignment
                if dist_sq < self.alignment_radius * self.alignment_radius {
                    align[0] += other_vel[0];
                    align[1] += other_vel[1];
                    align[2] += other_vel[2];
                    align_count += 1;
                }

                // Cohesion
                if dist_sq < self.cohesion_radius * self.cohesion_radius {
                    coh[0] += other_pos[0];
                    coh[1] += other_pos[1];
                    coh[2] += other_pos[2];
                    coh_count += 1;
                }
            }

            let mut acc = [0.0, 0.0, 0.0];

            // Apply separation
            if sep_count > 0 {
                sep[0] /= sep_count as f32;
                sep[1] /= sep_count as f32;
                sep[2] /= sep_count as f32;

                let mag = (sep[0] * sep[0] + sep[1] * sep[1] + sep[2] * sep[2]).sqrt();
                if mag > 0.0 {
                    sep[0] = (sep[0] / mag) * self.max_speed - boid.velocity[0];
                    sep[1] = (sep[1] / mag) * self.max_speed - boid.velocity[1];
                    sep[2] = (sep[2] / mag) * self.max_speed - boid.velocity[2];

                    acc[0] += sep[0] * self.separation_strength;
                    acc[1] += sep[1] * self.separation_strength;
                    acc[2] += sep[2] * self.separation_strength;
                }
            }

            // Apply alignment
            if align_count > 0 {
                align[0] /= align_count as f32;
                align[1] /= align_count as f32;
                align[2] /= align_count as f32;

                let mag = (align[0] * align[0] + align[1] * align[1] + align[2] * align[2]).sqrt();
                if mag > 0.0 {
                    align[0] = (align[0] / mag) * self.max_speed - boid.velocity[0];
                    align[1] = (align[1] / mag) * self.max_speed - boid.velocity[1];
                    align[2] = (align[2] / mag) * self.max_speed - boid.velocity[2];

                    acc[0] += align[0] * self.alignment_strength;
                    acc[1] += align[1] * self.alignment_strength;
                    acc[2] += align[2] * self.alignment_strength;
                }
            }

            // Apply cohesion
            if coh_count > 0 {
                coh[0] = coh[0] / coh_count as f32 - boid.position[0];
                coh[1] = coh[1] / coh_count as f32 - boid.position[1];
                coh[2] = coh[2] / coh_count as f32 - boid.position[2];

                let mag = (coh[0] * coh[0] + coh[1] * coh[1] + coh[2] * coh[2]).sqrt();
                if mag > 0.0 {
                    coh[0] = (coh[0] / mag) * self.max_speed - boid.velocity[0];
                    coh[1] = (coh[1] / mag) * self.max_speed - boid.velocity[1];
                    coh[2] = (coh[2] / mag) * self.max_speed - boid.velocity[2];

                    acc[0] += coh[0] * self.cohesion_strength;
                    acc[1] += coh[1] * self.cohesion_strength;
                    acc[2] += coh[2] * self.cohesion_strength;
                }
            }

            // Boundary force - keep boids in sphere
            let dist_from_center = (boid.position[0] * boid.position[0] +
                                   boid.position[1] * boid.position[1] +
                                   boid.position[2] * boid.position[2]).sqrt();

            if dist_from_center > self.bound_radius * 0.8 {
                let strength = (dist_from_center - self.bound_radius * 0.8) / (self.bound_radius * 0.2);
                acc[0] -= boid.position[0] / dist_from_center * strength * 2.0;
                acc[1] -= boid.position[1] / dist_from_center * strength * 2.0;
                acc[2] -= boid.position[2] / dist_from_center * strength * 2.0;
            }

            // Limit force
            let force_mag = (acc[0] * acc[0] + acc[1] * acc[1] + acc[2] * acc[2]).sqrt();
            if force_mag > self.max_force {
                acc[0] = (acc[0] / force_mag) * self.max_force;
                acc[1] = (acc[1] / force_mag) * self.max_force;
                acc[2] = (acc[2] / force_mag) * self.max_force;
            }

            // Update velocity
            boid.velocity[0] += acc[0] * dt;
            boid.velocity[1] += acc[1] * dt;
            boid.velocity[2] += acc[2] * dt;

            // Limit speed
            let speed = (boid.velocity[0] * boid.velocity[0] +
                        boid.velocity[1] * boid.velocity[1] +
                        boid.velocity[2] * boid.velocity[2]).sqrt();
            if speed > self.max_speed {
                boid.velocity[0] = (boid.velocity[0] / speed) * self.max_speed;
                boid.velocity[1] = (boid.velocity[1] / speed) * self.max_speed;
                boid.velocity[2] = (boid.velocity[2] / speed) * self.max_speed;
            }

            // Update position
            boid.position[0] += boid.velocity[0] * dt;
            boid.position[1] += boid.velocity[1] * dt;
            boid.position[2] += boid.velocity[2] * dt;
        }
    }
}

impl Simulation3D for Boids3D {
    fn name(&self) -> &str {
        "3D Boids Flocking"
    }

    fn step(&mut self, dt: f32) {
        let dt = dt * self.speed;
        self.update_boids(dt);
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.boids.iter().map(|b| b.position).collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("3D Boids Flocking");

        ui.label(format!("Boid count: {}", self.boids.len()));

        egui::CollapsingHeader::new("🐦 Boid Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.boid_count, 10..=150)
                    .text("Boid Count")).changed() {
                    self.init_boids();
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.max_speed, 0.5..=5.0)
                    .text("Max Speed")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.max_force, 0.01..=0.5)
                    .text("Max Force")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.bound_radius, 10.0..=50.0)
                    .text("Bound Radius")).changed();
            });

        egui::CollapsingHeader::new("🎯 Flocking Behavior")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.separation_radius, 1.0..=15.0)
                    .text("Separation Radius")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.alignment_radius, 5.0..=25.0)
                    .text("Alignment Radius")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.cohesion_radius, 5.0..=25.0)
                    .text("Cohesion Radius")).changed();

                ui.separator();

                changed |= ui.add(egui::Slider::new(&mut self.separation_strength, 0.0..=5.0)
                    .text("Separation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.alignment_strength, 0.0..=5.0)
                    .text("Alignment")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.cohesion_strength, 0.0..=5.0)
                    .text("Cohesion")).changed();
            });

        ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0)
            .text("Speed"));

        if ui.button("🔄 Reset").clicked() {
            self.reset();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.init_boids();
    }
}
