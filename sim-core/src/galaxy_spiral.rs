use crate::Simulation3D;

#[allow(dead_code)]
struct Star {
    position: [f32; 3],
    velocity: [f32; 3],
    orbit_radius: f32,
    orbit_angle: f32,
    orbit_speed: f32,
    z_offset: f32,
}

pub struct GalaxySpiral {
    pub num_arms: usize,
    pub stars_per_arm: usize,
    pub arm_spread: f32,
    pub rotation_speed: f32,
    pub core_radius: f32,
    pub max_radius: f32,
    pub disk_thickness: f32,
    pub show_core: bool,
    pub orbital_velocity_falloff: f32,

    stars: Vec<Star>,
    animation_time: f32,
}

impl Default for GalaxySpiral {
    fn default() -> Self {
        Self {
            num_arms: 4,
            stars_per_arm: 200,
            arm_spread: 0.3,
            rotation_speed: 0.5,
            core_radius: 5.0,
            max_radius: 40.0,
            disk_thickness: 8.0,
            show_core: true,
            orbital_velocity_falloff: 0.8,
            stars: Vec::new(),
            animation_time: 0.0,
        }
    }
}

impl GalaxySpiral {
    pub fn new() -> Self {
        let mut galaxy = Self::default();
        galaxy.regenerate();
        galaxy
    }

    fn regenerate(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        self.stars.clear();
        let random_state = RandomState::new();

        for arm in 0..self.num_arms {
            let arm_angle = (arm as f32 / self.num_arms as f32) * std::f32::consts::TAU;

            for i in 0..self.stars_per_arm {
                let mut hasher = random_state.build_hasher();
                (arm * 10000 + i).hash(&mut hasher);
                let hash1 = hasher.finish();

                let mut hasher = random_state.build_hasher();
                (arm * 10000 + i + 1).hash(&mut hasher);
                let hash2 = hasher.finish();

                let mut hasher = random_state.build_hasher();
                (arm * 10000 + i + 2).hash(&mut hasher);
                let hash3 = hasher.finish();

                // Normalized position along arm (0 to 1)
                let t = i as f32 / self.stars_per_arm as f32;

                // Radius from center - more stars towards the edge
                let orbit_radius = self.core_radius + (self.max_radius - self.core_radius) * t * t;

                // Spiral angle - increases with radius
                let spiral_angle = arm_angle + t * self.arm_spread * std::f32::consts::TAU;

                // Add randomness
                let random_angle = ((hash1 % 1000) as f32 / 1000.0 - 0.5) * 0.3;
                let random_radius = ((hash2 % 1000) as f32 / 1000.0 - 0.5) * 3.0;
                let z_random = ((hash3 % 1000) as f32 / 1000.0 - 0.5) * self.disk_thickness;

                let final_angle = spiral_angle + random_angle;
                let final_radius = (orbit_radius + random_radius).max(1.0);

                let x = final_radius * final_angle.cos();
                let y = final_radius * final_angle.sin();
                let z = z_random * (1.0 - t * 0.3); // Thinner at edges

                // Orbital speed decreases with radius (like real galaxies)
                let orbit_speed = self.rotation_speed * (1.0 / (1.0 + orbit_radius * self.orbital_velocity_falloff));

                self.stars.push(Star {
                    position: [x, y, z],
                    velocity: [0.0, 0.0, 0.0],
                    orbit_radius: final_radius,
                    orbit_angle: final_angle,
                    orbit_speed,
                    z_offset: z,
                });
            }
        }

        // Add bright core stars
        if self.show_core {
            for i in 0..100 {
                let mut hasher = random_state.build_hasher();
                (999000 + i).hash(&mut hasher);
                let hash1 = hasher.finish();

                let mut hasher = random_state.build_hasher();
                (999000 + i + 1).hash(&mut hasher);
                let hash2 = hasher.finish();

                let mut hasher = random_state.build_hasher();
                (999000 + i + 2).hash(&mut hasher);
                let hash3 = hasher.finish();

                let theta = ((hash1 % 1000) as f32 / 1000.0) * std::f32::consts::TAU;
                let phi = ((hash2 % 1000) as f32 / 1000.0) * std::f32::consts::PI;
                let r = ((hash3 % 1000) as f32 / 1000.0) * self.core_radius;

                let x = r * phi.sin() * theta.cos();
                let y = r * phi.sin() * theta.sin();
                let z = r * phi.cos() * 0.3;

                self.stars.push(Star {
                    position: [x, y, z],
                    velocity: [0.0, 0.0, 0.0],
                    orbit_radius: r,
                    orbit_angle: theta,
                    orbit_speed: self.rotation_speed * 2.0,
                    z_offset: z,
                });
            }
        }
    }

    fn update_stars(&mut self, dt: f32) {
        for star in &mut self.stars {
            // Update orbital angle
            star.orbit_angle += star.orbit_speed * dt;

            // Calculate new position
            star.position[0] = star.orbit_radius * star.orbit_angle.cos();
            star.position[1] = star.orbit_radius * star.orbit_angle.sin();
            // Z oscillates slightly
            star.position[2] = star.z_offset * (self.animation_time * 0.5 + star.orbit_angle).sin() * 0.2 + star.z_offset;
        }
    }
}

impl Simulation3D for GalaxySpiral {
    fn name(&self) -> &str {
        "Galaxy Spiral"
    }

    fn step(&mut self, dt: f32) {
        self.animation_time += dt;
        self.update_stars(dt);
    }

    fn get_points(&self) -> Vec<[f32; 3]> {
        self.stars.iter().map(|s| s.position).collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Galaxy Spiral");

        ui.label(format!("Total stars: {}", self.stars.len()));

        egui::CollapsingHeader::new("🌌 Galaxy Structure")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.num_arms, 2..=8)
                    .text("Number of Arms")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.stars_per_arm, 50..=500)
                    .text("Stars per Arm")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.arm_spread, 0.1..=1.0)
                    .text("Arm Spread")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.core_radius, 2.0..=15.0)
                    .text("Core Radius")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.max_radius, 20.0..=80.0)
                    .text("Max Radius")).changed() {
                    self.regenerate();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.disk_thickness, 2.0..=20.0)
                    .text("Disk Thickness")).changed() {
                    self.regenerate();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("⚙ Physics")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.rotation_speed, 0.0..=2.0)
                    .text("Rotation Speed"));

                if ui.add(egui::Slider::new(&mut self.orbital_velocity_falloff, 0.0..=2.0)
                    .text("Velocity Falloff")).changed() {
                    self.regenerate();
                    changed = true;
                }

                ui.label("Lower falloff = flatter rotation curve");
                ui.label("(like dark matter halos)");
            });

        egui::CollapsingHeader::new("🎨 Visual")
            .show(ui, |ui| {
                if ui.checkbox(&mut self.show_core, "Show Bright Core").changed() {
                    self.regenerate();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Milky Way-like (4 arms)").clicked() {
                    self.num_arms = 4;
                    self.arm_spread = 0.3;
                    self.orbital_velocity_falloff = 0.6;
                    self.regenerate();
                    changed = true;
                }
                if ui.button("Whirlpool (2 arms)").clicked() {
                    self.num_arms = 2;
                    self.arm_spread = 0.5;
                    self.orbital_velocity_falloff = 0.8;
                    self.regenerate();
                    changed = true;
                }
                if ui.button("Pinwheel (5 arms)").clicked() {
                    self.num_arms = 5;
                    self.arm_spread = 0.2;
                    self.orbital_velocity_falloff = 0.7;
                    self.regenerate();
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            *self = Self::new();
            changed = true;
        }

        changed
    }

    fn reset(&mut self) {
        self.animation_time = 0.0;
        self.regenerate();
    }
}
