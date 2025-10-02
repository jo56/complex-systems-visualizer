use crate::{Color, ColorScheme, Simulation2D};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    lifetime: f32,
    max_lifetime: f32,
    size: f32,
    color: Color,
}

pub struct PerlinFlow {
    pub particle_count: usize,
    pub noise_scale: f32,
    pub noise_speed: f32,
    pub flow_strength: f32,
    pub particle_speed: f32,
    pub particle_size_min: f32,
    pub particle_size_max: f32,
    pub particle_lifetime_min: f32,
    pub particle_lifetime_max: f32,
    pub color_scheme: ColorScheme,
    pub color_by_velocity: bool,
    pub color_by_direction: bool,
    pub color_by_lifetime: bool,
    pub fade_by_lifetime: bool,
    pub show_trails: bool,
    pub trail_length: usize,
    pub spawn_mode: SpawnMode,
    pub wrap_edges: bool,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub animate: bool,
    pub animation_speed: f32,
    pub noise_octaves: usize,
    pub noise_persistence: f32,
    pub noise_z_offset: f32,
    pub velocity_damping: f32,
    pub particle_glow: bool,
    pub background_alpha: f32,
    particles: Vec<Particle>,
    noise: Perlin,
    animation_time: f32,
    trail_history: Vec<Vec<(f32, f32)>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SpawnMode {
    Random,
    Center,
    Edges,
    Grid,
}

impl SpawnMode {
    pub fn all() -> Vec<SpawnMode> {
        vec![
            SpawnMode::Random,
            SpawnMode::Center,
            SpawnMode::Edges,
            SpawnMode::Grid,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            SpawnMode::Random => "Random",
            SpawnMode::Center => "Center",
            SpawnMode::Edges => "Edges",
            SpawnMode::Grid => "Grid",
        }
    }
}

impl Default for PerlinFlow {
    fn default() -> Self {
        Self {
            particle_count: 1000,
            noise_scale: 0.003,
            noise_speed: 0.5,
            flow_strength: 2.0,
            particle_speed: 1.0,
            particle_size_min: 1.0,
            particle_size_max: 3.0,
            particle_lifetime_min: 2.0,
            particle_lifetime_max: 5.0,
            color_scheme: ColorScheme::Rainbow,
            color_by_velocity: false,
            color_by_direction: true,
            color_by_lifetime: false,
            fade_by_lifetime: true,
            show_trails: false,
            trail_length: 10,
            spawn_mode: SpawnMode::Random,
            wrap_edges: true,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            animate: true,
            animation_speed: 1.0,
            noise_octaves: 1,
            noise_persistence: 0.5,
            noise_z_offset: 0.0,
            velocity_damping: 0.95,
            particle_glow: false,
            background_alpha: 0.1,
            particles: Vec::new(),
            noise: Perlin::new(42),
            animation_time: 0.0,
            trail_history: Vec::new(),
        }
    }
}

impl PerlinFlow {
    pub fn new() -> Self {
        let mut flow = Self::default();
        flow.init_particles(800, 600);
        flow
    }

    fn init_particles(&mut self, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        self.particles.clear();
        self.trail_history.clear();

        for _ in 0..self.particle_count {
            let (x, y) = match self.spawn_mode {
                SpawnMode::Random => {
                    (rng.gen_range(0.0..width as f32), rng.gen_range(0.0..height as f32))
                }
                SpawnMode::Center => {
                    let cx = width as f32 / 2.0;
                    let cy = height as f32 / 2.0;
                    let radius = rng.gen_range(0.0..50.0);
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    (cx + radius * angle.cos(), cy + radius * angle.sin())
                }
                SpawnMode::Edges => {
                    if rng.gen_bool(0.5) {
                        (rng.gen_range(0.0..width as f32), if rng.gen_bool(0.5) { 0.0 } else { height as f32 })
                    } else {
                        (if rng.gen_bool(0.5) { 0.0 } else { width as f32 }, rng.gen_range(0.0..height as f32))
                    }
                }
                SpawnMode::Grid => {
                    let cols = (self.particle_count as f32).sqrt() as usize;
                    let rows = (self.particle_count + cols - 1) / cols;
                    let i = self.particles.len();
                    let col = i % cols;
                    let row = i / cols;
                    (
                        (col as f32 + 0.5) * width as f32 / cols as f32,
                        (row as f32 + 0.5) * height as f32 / rows as f32
                    )
                }
            };

            let lifetime = rng.gen_range(self.particle_lifetime_min..self.particle_lifetime_max);
            let size = rng.gen_range(self.particle_size_min..self.particle_size_max);

            self.particles.push(Particle {
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                lifetime,
                max_lifetime: lifetime,
                size,
                color: Color::WHITE,
            });

            if self.show_trails {
                self.trail_history.push(Vec::new());
            }
        }
    }

    fn update_particles(&mut self, width: usize, height: usize, dt: f32) {
        let mut rng = rand::thread_rng();
        let time_offset = if self.animate {
            self.animation_time * self.animation_speed
        } else {
            self.noise_z_offset
        };

        // Get parameters to avoid borrow issues
        let color_scheme = self.color_scheme;
        let hue_shift = self.hue_shift;
        let saturation = self.saturation;
        let brightness = self.brightness;

        for (i, particle) in self.particles.iter_mut().enumerate() {
            // Get flow direction from noise
            let noise_val = self.noise.get([
                particle.x as f64 * self.noise_scale as f64,
                particle.y as f64 * self.noise_scale as f64,
                time_offset as f64,
            ]) as f32;

            let angle = noise_val * std::f32::consts::TAU;

            // Apply flow force
            particle.vx += angle.cos() * self.flow_strength * dt;
            particle.vy += angle.sin() * self.flow_strength * dt;

            // Apply damping
            particle.vx *= self.velocity_damping;
            particle.vy *= self.velocity_damping;

            // Update position
            particle.x += particle.vx * self.particle_speed * dt * 60.0;
            particle.y += particle.vy * self.particle_speed * dt * 60.0;

            // Update trail
            if self.show_trails && i < self.trail_history.len() {
                self.trail_history[i].push((particle.x, particle.y));
                if self.trail_history[i].len() > self.trail_length {
                    self.trail_history[i].remove(0);
                }
            }

            // Handle edges
            if self.wrap_edges {
                if particle.x < 0.0 { particle.x += width as f32; }
                if particle.x >= width as f32 { particle.x -= width as f32; }
                if particle.y < 0.0 { particle.y += height as f32; }
                if particle.y >= height as f32 { particle.y -= height as f32; }
            } else {
                if particle.x < 0.0 || particle.x >= width as f32 ||
                   particle.y < 0.0 || particle.y >= height as f32 {
                    particle.lifetime = 0.0;
                }
            }

            // Update lifetime
            particle.lifetime -= dt;

            // Respawn if dead
            if particle.lifetime <= 0.0 {
                let (x, y) = match self.spawn_mode {
                    SpawnMode::Random => {
                        (rng.gen_range(0.0..width as f32), rng.gen_range(0.0..height as f32))
                    }
                    SpawnMode::Center => {
                        let cx = width as f32 / 2.0;
                        let cy = height as f32 / 2.0;
                        let radius = rng.gen_range(0.0..50.0);
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        (cx + radius * angle.cos(), cy + radius * angle.sin())
                    }
                    SpawnMode::Edges => {
                        if rng.gen_bool(0.5) {
                            (rng.gen_range(0.0..width as f32), if rng.gen_bool(0.5) { 0.0 } else { height as f32 })
                        } else {
                            (if rng.gen_bool(0.5) { 0.0 } else { width as f32 }, rng.gen_range(0.0..height as f32))
                        }
                    }
                    SpawnMode::Grid => (particle.x, particle.y),
                };

                particle.x = x;
                particle.y = y;
                particle.vx = 0.0;
                particle.vy = 0.0;
                particle.lifetime = rng.gen_range(self.particle_lifetime_min..self.particle_lifetime_max);
                particle.max_lifetime = particle.lifetime;

                if self.show_trails && i < self.trail_history.len() {
                    self.trail_history[i].clear();
                }
            }

            // Update color
            let t = if self.color_by_velocity {
                let speed = (particle.vx * particle.vx + particle.vy * particle.vy).sqrt();
                (speed / 10.0).min(1.0)
            } else if self.color_by_direction {
                let angle = particle.vy.atan2(particle.vx);
                (angle + std::f32::consts::PI) / std::f32::consts::TAU
            } else if self.color_by_lifetime {
                particle.lifetime / particle.max_lifetime
            } else {
                i as f32 / self.particle_count as f32
            };

            let mut color = color_scheme.map(t, true);

            // Apply color adjustments inline
            let (h, s, v) = Color::rgb_to_hsv_static(color);
            let new_h = (h + hue_shift * 360.0) % 360.0;
            let new_s = (s * saturation).clamp(0.0, 1.0);
            let new_v = (v * brightness).clamp(0.0, 1.0);
            color = Color::from_hsv(new_h, new_s, new_v);

            particle.color = color;
        }
    }

    fn apply_color_adjustments(&self, mut color: Color) -> Color {
        let (h, s, v) = self.rgb_to_hsv(color);
        let new_h = (h + self.hue_shift * 360.0) % 360.0;
        let new_s = (s * self.saturation).clamp(0.0, 1.0);
        let new_v = (v * self.brightness).clamp(0.0, 1.0);

        Color::from_hsv(new_h, new_s, new_v)
    }

    fn rgb_to_hsv(&self, color: Color) -> (f32, f32, f32) {
        let r = color.r as f32 / 255.0;
        let g = color.g as f32 / 255.0;
        let b = color.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    fn draw_circle(&self, pixels: &mut [Color], width: usize, height: usize,
                   cx: f32, cy: f32, radius: f32, color: Color) {
        let r_sq = radius * radius;
        let min_x = (cx - radius).max(0.0) as usize;
        let max_x = (cx + radius).min(width as f32 - 1.0) as usize;
        let min_y = (cy - radius).max(0.0) as usize;
        let max_y = (cy + radius).min(height as f32 - 1.0) as usize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= r_sq {
                    let idx = y * width + x;
                    if self.particle_glow {
                        // Additive blending for glow effect
                        let old = pixels[idx];
                        pixels[idx] = Color::from_rgb(
                            (old.r as u16 + color.r as u16).min(255) as u8,
                            (old.g as u16 + color.g as u16).min(255) as u8,
                            (old.b as u16 + color.b as u16).min(255) as u8,
                        );
                    } else {
                        pixels[idx] = color;
                    }
                }
            }
        }
    }
}

impl Simulation2D for PerlinFlow {
    fn name(&self) -> &str {
        "Perlin Flow Field"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        // Apply background fade for trail effect
        if self.show_trails && self.background_alpha < 1.0 {
            for pixel in pixels.iter_mut() {
                let fade = (255.0 * self.background_alpha) as u8;
                *pixel = Color::from_rgb(
                    ((pixel.r as u16 * fade as u16) / 255) as u8,
                    ((pixel.g as u16 * fade as u16) / 255) as u8,
                    ((pixel.b as u16 * fade as u16) / 255) as u8,
                );
            }
        }

        // Draw trails
        if self.show_trails {
            for trail in &self.trail_history {
                for i in 0..trail.len().saturating_sub(1) {
                    let (x1, y1) = trail[i];
                    let (x2, y2) = trail[i + 1];

                    if x1 >= 0.0 && x1 < width as f32 && y1 >= 0.0 && y1 < height as f32 {
                        let alpha = i as f32 / trail.len() as f32;
                        let color = Color::from_rgb(
                            (255.0 * alpha) as u8,
                            (255.0 * alpha) as u8,
                            (255.0 * alpha) as u8,
                        );
                        self.draw_circle(&mut pixels, width, height, x1, y1, 1.0, color);
                    }
                }
            }
        }

        // Draw particles
        for particle in &self.particles {
            if particle.x < 0.0 || particle.x >= width as f32 ||
               particle.y < 0.0 || particle.y >= height as f32 {
                continue;
            }

            let mut color = particle.color;

            // Apply lifetime fade
            if self.fade_by_lifetime {
                let alpha = particle.lifetime / particle.max_lifetime;
                color = Color::from_rgb(
                    ((color.r as f32 * alpha) as u8),
                    ((color.g as f32 * alpha) as u8),
                    ((color.b as f32 * alpha) as u8),
                );
            }

            self.draw_circle(&mut pixels, width, height, particle.x, particle.y, particle.size, color);
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Perlin Flow Field");

        egui::CollapsingHeader::new("⚙ Particle Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.particle_count, 10..=5000)
                    .text("Particle Count")).changed() {
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.particle_speed, 0.1..=5.0)
                    .text("Speed")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.particle_size_min, 0.5..=10.0)
                    .text("Min Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.particle_size_max, 0.5..=20.0)
                    .text("Max Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.particle_lifetime_min, 0.5..=10.0)
                    .text("Min Lifetime")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.particle_lifetime_max, 0.5..=20.0)
                    .text("Max Lifetime")).changed();

                egui::ComboBox::from_label("Spawn Mode")
                    .selected_text(self.spawn_mode.name())
                    .show_ui(ui, |ui| {
                        for mode in SpawnMode::all() {
                            if ui.selectable_value(&mut self.spawn_mode, mode, mode.name()).clicked() {
                                changed = true;
                            }
                        }
                    });
            });

        egui::CollapsingHeader::new("🌊 Flow Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.noise_scale, 0.0001..=0.01)
                    .text("Noise Scale")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.flow_strength, 0.1..=10.0)
                    .text("Flow Strength")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.velocity_damping, 0.8..=0.99)
                    .text("Damping")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.noise_octaves, 1..=4)
                    .text("Noise Octaves")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.noise_persistence, 0.1..=1.0)
                    .text("Persistence")).changed();

                changed |= ui.checkbox(&mut self.wrap_edges, "Wrap Edges").changed();
            });

        egui::CollapsingHeader::new("🎨 Color Settings")
            .default_open(true)
            .show(ui, |ui| {
                egui::ComboBox::from_label("Color Scheme")
                    .selected_text(self.color_scheme.name())
                    .show_ui(ui, |ui| {
                        for scheme in ColorScheme::all() {
                            if ui.selectable_value(&mut self.color_scheme, scheme, scheme.name()).clicked() {
                                changed = true;
                            }
                        }
                    });

                changed |= ui.checkbox(&mut self.color_by_velocity, "Color by Velocity").changed();
                changed |= ui.checkbox(&mut self.color_by_direction, "Color by Direction").changed();
                changed |= ui.checkbox(&mut self.color_by_lifetime, "Color by Lifetime").changed();
                changed |= ui.checkbox(&mut self.fade_by_lifetime, "Fade by Lifetime").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();
            });

        egui::CollapsingHeader::new("✨ Visual Effects")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_trails, "Show Trails").changed();

                if self.show_trails {
                    changed |= ui.add(egui::Slider::new(&mut self.trail_length, 2..=50)
                        .text("Trail Length")).changed();

                    changed |= ui.add(egui::Slider::new(&mut self.background_alpha, 0.0..=1.0)
                        .text("Background Fade")).changed();
                }

                changed |= ui.checkbox(&mut self.particle_glow, "Particle Glow").changed();
            });

        egui::CollapsingHeader::new("🎬 Animation")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.animate, "Enable Animation").changed();

                if self.animate {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                } else {
                    changed |= ui.add(egui::Slider::new(&mut self.noise_z_offset, 0.0..=100.0)
                        .text("Noise Offset")).changed();
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Flowing Rivers").clicked() {
                    *self = Self::default();
                    self.color_scheme = ColorScheme::Ocean;
                    self.particle_count = 2000;
                    self.show_trails = true;
                    changed = true;
                }
                if ui.button("Rainbow Currents").clicked() {
                    self.color_scheme = ColorScheme::Rainbow;
                    self.particle_count = 1500;
                    self.color_by_direction = true;
                    self.show_trails = true;
                    changed = true;
                }
                if ui.button("Fire Storm").clicked() {
                    self.color_scheme = ColorScheme::Fire;
                    self.particle_count = 3000;
                    self.flow_strength = 5.0;
                    self.particle_glow = true;
                    changed = true;
                }
                if ui.button("Cosmic Dust").clicked() {
                    self.color_scheme = ColorScheme::Galaxy;
                    self.particle_count = 5000;
                    self.particle_size_min = 0.5;
                    self.particle_size_max = 2.0;
                    self.particle_glow = true;
                    changed = true;
                }
            });

        // Handle animation and particle updates
        let dt = ui.input(|i| i.stable_dt);
        if self.animate {
            self.animation_time += dt;
            changed = true;
        }

        // Reinitialize particles if count changed
        let current_count = self.particles.len();
        if current_count != self.particle_count {
            let size = ui.available_size();
            self.init_particles(size.x as usize, size.y as usize);
            changed = true;
        }

        // Update particles every frame
        let size = ui.available_size();
        self.update_particles(size.x.max(800.0) as usize, size.y.max(600.0) as usize, dt);
        changed = true;

        changed
    }
}
