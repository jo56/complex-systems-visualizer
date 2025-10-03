use crate::{Color, ColorScheme, Simulation2D};
use rand::Rng;

struct Agent {
    x: f32,
    y: f32,
    angle: f32,
}

pub struct SlimeMold {
    pub agent_count: usize,
    pub sensor_angle: f32,
    pub sensor_distance: f32,
    pub turn_angle: f32,
    pub move_speed: f32,
    pub deposit_amount: f32,
    pub decay_rate: f32,
    pub diffuse_rate: f32,
    pub color_scheme: ColorScheme,
    pub trail_brightness: f32,
    agents: Vec<Agent>,
    trail_map: Vec<f32>,
    width: usize,
    height: usize,
}

impl Default for SlimeMold {
    fn default() -> Self {
        Self {
            agent_count: 5000,
            sensor_angle: 0.4,
            sensor_distance: 9.0,
            turn_angle: 0.4,
            move_speed: 1.0,
            deposit_amount: 5.0,
            decay_rate: 0.1,
            diffuse_rate: 0.5,
            color_scheme: ColorScheme::Viridis,
            trail_brightness: 1.0,
            agents: Vec::new(),
            trail_map: Vec::new(),
            width: 800,
            height: 600,
        }
    }
}

impl SlimeMold {
    pub fn new() -> Self {
        let mut sim = Self::default();
        sim.init_agents(800, 600);
        sim
    }

    fn init_agents(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.trail_map = vec![0.0; width * height];
        self.agents.clear();

        let mut rng = rand::thread_rng();
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        for _ in 0..self.agent_count {
            // Spawn in center with random angles
            let radius = rng.gen_range(0.0..50.0);
            let spawn_angle = rng.gen_range(0.0..std::f32::consts::TAU);

            self.agents.push(Agent {
                x: cx + radius * spawn_angle.cos(),
                y: cy + radius * spawn_angle.sin(),
                angle: rng.gen_range(0.0..std::f32::consts::TAU),
            });
        }
    }

    fn sense(&self, x: f32, y: f32, angle: f32) -> f32 {
        let sensor_x = x + angle.cos() * self.sensor_distance;
        let sensor_y = y + angle.sin() * self.sensor_distance;

        if sensor_x < 0.0 || sensor_x >= self.width as f32 ||
           sensor_y < 0.0 || sensor_y >= self.height as f32 {
            return 0.0;
        }

        let ix = sensor_x as usize;
        let iy = sensor_y as usize;
        let idx = iy * self.width + ix;

        if idx < self.trail_map.len() {
            self.trail_map[idx]
        } else {
            0.0
        }
    }

    fn update_agents(&mut self) {
        let mut rng = rand::thread_rng();

        // Store sensor readings for all agents
        let sensor_data: Vec<(f32, f32, f32)> = self.agents.iter()
            .map(|agent| {
                let forward = self.sense(agent.x, agent.y, agent.angle);
                let left = self.sense(agent.x, agent.y, agent.angle - self.sensor_angle);
                let right = self.sense(agent.x, agent.y, agent.angle + self.sensor_angle);
                (forward, left, right)
            })
            .collect();

        for (i, agent) in self.agents.iter_mut().enumerate() {
            let (forward, left, right) = sensor_data[i];

            // Determine turn direction
            if forward > left && forward > right {
                // Continue forward
            } else if forward < left && forward < right {
                // Random turn
                agent.angle += if rng.gen_bool(0.5) {
                    self.turn_angle
                } else {
                    -self.turn_angle
                };
            } else if left > right {
                agent.angle -= self.turn_angle;
            } else if right > left {
                agent.angle += self.turn_angle;
            }

            // Move forward
            let new_x = agent.x + agent.angle.cos() * self.move_speed;
            let new_y = agent.y + agent.angle.sin() * self.move_speed;

            // Bounce off edges
            if new_x < 0.0 || new_x >= self.width as f32 {
                agent.angle = std::f32::consts::PI - agent.angle;
            } else if new_y < 0.0 || new_y >= self.height as f32 {
                agent.angle = -agent.angle;
            } else {
                agent.x = new_x;
                agent.y = new_y;
            }

            // Deposit trail
            let ix = agent.x as usize;
            let iy = agent.y as usize;
            if ix < self.width && iy < self.height {
                let idx = iy * self.width + ix;
                if idx < self.trail_map.len() {
                    self.trail_map[idx] = (self.trail_map[idx] + self.deposit_amount).min(255.0);
                }
            }
        }
    }

    fn diffuse_and_decay(&mut self) {
        let mut new_map = vec![0.0; self.width * self.height];

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;

                // Simple box blur for diffusion
                let sum = self.trail_map[idx - 1] +
                          self.trail_map[idx + 1] +
                          self.trail_map[idx - self.width] +
                          self.trail_map[idx + self.width] +
                          self.trail_map[idx] * 4.0;

                let blurred = sum / 8.0;
                new_map[idx] = (blurred * (1.0 - self.decay_rate)).max(0.0);
            }
        }

        self.trail_map = new_map;
    }
}

impl Simulation2D for SlimeMold {
    fn name(&self) -> &str {
        "Slime Mold Simulation"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        // Scale trail map to requested size
        for y in 0..height {
            for x in 0..width {
                let src_x = (x * self.width) / width;
                let src_y = (y * self.height) / height;
                let src_idx = src_y * self.width + src_x;

                if src_idx < self.trail_map.len() {
                    let intensity = (self.trail_map[src_idx] / 255.0 * self.trail_brightness).min(1.0);
                    pixels[y * width + x] = self.color_scheme.map(intensity, true);
                }
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Slime Mold Simulation");

        egui::CollapsingHeader::new("🦠 Agent Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.agent_count, 100..=10000)
                    .text("Agent Count")).changed() {
                    self.init_agents(self.width, self.height);
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.move_speed, 0.1..=5.0)
                    .text("Move Speed"));

                ui.add(egui::Slider::new(&mut self.sensor_distance, 1.0..=20.0)
                    .text("Sensor Distance"));

                ui.add(egui::Slider::new(&mut self.sensor_angle, 0.1..=1.5)
                    .text("Sensor Angle"));

                ui.add(egui::Slider::new(&mut self.turn_angle, 0.1..=1.5)
                    .text("Turn Angle"));
            });

        egui::CollapsingHeader::new("🎨 Trail Settings")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.deposit_amount, 1.0..=20.0)
                    .text("Deposit Amount"));

                ui.add(egui::Slider::new(&mut self.decay_rate, 0.01..=0.5)
                    .text("Decay Rate"));

                ui.add(egui::Slider::new(&mut self.trail_brightness, 0.5..=3.0)
                    .text("Brightness"));

                egui::ComboBox::from_label("Color Scheme")
                    .selected_text(self.color_scheme.name())
                    .show_ui(ui, |ui| {
                        for scheme in ColorScheme::all() {
                            if ui.selectable_value(&mut self.color_scheme, scheme, scheme.name()).clicked() {
                                changed = true;
                            }
                        }
                    });
            });

        if ui.button("🔄 Reset").clicked() {
            self.init_agents(self.width, self.height);
        }

        // Update simulation every frame
        self.update_agents();
        self.diffuse_and_decay();

        true
    }
}
