use crate::{Color, ColorScheme, Simulation2D};
use rand::Rng;

pub struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    size: f32,
}

pub struct Boids {
    pub boid_count: usize,
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,
    pub separation_strength: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub boid_size_min: f32,
    pub boid_size_max: f32,
    pub color_scheme: ColorScheme,
    pub color_by_velocity: bool,
    pub color_by_neighbors: bool,
    pub color_by_direction: bool,
    pub show_velocity_lines: bool,
    pub show_perception_radius: bool,
    pub wrap_edges: bool,
    pub avoid_mouse: bool,
    pub mouse_avoidance_radius: f32,
    pub mouse_avoidance_strength: f32,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub background_trails: bool,
    pub trail_fade: f32,
    pub draw_triangles: bool,
    pub flock_mode: FlockMode,
    pub predator_count: usize,
    pub predator_radius: f32,
    pub predator_strength: f32,
    boids: Vec<Boid>,
    predators: Vec<Boid>,
    mouse_x: f32,
    mouse_y: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FlockMode {
    Normal,
    Chaotic,
    Ordered,
    Swirl,
}

impl FlockMode {
    pub fn all() -> Vec<FlockMode> {
        vec![
            FlockMode::Normal,
            FlockMode::Chaotic,
            FlockMode::Ordered,
            FlockMode::Swirl,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            FlockMode::Normal => "Normal",
            FlockMode::Chaotic => "Chaotic",
            FlockMode::Ordered => "Ordered",
            FlockMode::Swirl => "Swirl",
        }
    }
}

impl Default for Boids {
    fn default() -> Self {
        Self {
            boid_count: 150,
            separation_radius: 25.0,
            alignment_radius: 50.0,
            cohesion_radius: 50.0,
            separation_strength: 1.5,
            alignment_strength: 1.0,
            cohesion_strength: 1.0,
            max_speed: 4.0,
            max_force: 0.3,
            boid_size_min: 3.0,
            boid_size_max: 5.0,
            color_scheme: ColorScheme::Rainbow,
            color_by_velocity: false,
            color_by_neighbors: false,
            color_by_direction: true,
            show_velocity_lines: false,
            show_perception_radius: false,
            wrap_edges: true,
            avoid_mouse: false,
            mouse_avoidance_radius: 100.0,
            mouse_avoidance_strength: 2.0,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            background_trails: false,
            trail_fade: 0.1,
            draw_triangles: true,
            flock_mode: FlockMode::Normal,
            predator_count: 0,
            predator_radius: 100.0,
            predator_strength: 3.0,
            boids: Vec::new(),
            predators: Vec::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

impl Boids {
    pub fn new() -> Self {
        let mut boids = Self::default();
        boids.init_boids(800, 600);
        boids
    }

    fn init_boids(&mut self, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        self.boids.clear();
        self.predators.clear();

        for _ in 0..self.boid_count {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(1.0..self.max_speed);

            self.boids.push(Boid {
                x: rng.gen_range(0.0..width as f32),
                y: rng.gen_range(0.0..height as f32),
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                color: Color::WHITE,
                size: rng.gen_range(self.boid_size_min..self.boid_size_max),
            });
        }

        for _ in 0..self.predator_count {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = self.max_speed * 0.8;

            self.predators.push(Boid {
                x: rng.gen_range(0.0..width as f32),
                y: rng.gen_range(0.0..height as f32),
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                color: Color::RED,
                size: 8.0,
            });
        }
    }

    fn update_boids(&mut self, width: usize, height: usize, _dt: f32) {
        let boid_positions: Vec<(f32, f32, f32, f32)> = self.boids
            .iter()
            .map(|b| (b.x, b.y, b.vx, b.vy))
            .collect();

        let predator_positions: Vec<(f32, f32)> = self.predators
            .iter()
            .map(|p| (p.x, p.y))
            .collect();

        // Get parameters to avoid borrow issues
        let color_scheme = self.color_scheme;
        let hue_shift = self.hue_shift;
        let saturation = self.saturation;
        let brightness = self.brightness;

        // Update regular boids
        for (i, boid) in self.boids.iter_mut().enumerate() {
            let mut sep_x = 0.0;
            let mut sep_y = 0.0;
            let mut sep_count = 0;

            let mut align_x = 0.0;
            let mut align_y = 0.0;
            let mut align_count = 0;

            let mut coh_x = 0.0;
            let mut coh_y = 0.0;
            let mut coh_count = 0;

            // Calculate flocking forces
            for (j, &(ox, oy, ovx, ovy)) in boid_positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dx = ox - boid.x;
                let dy = oy - boid.y;
                let dist_sq = dx * dx + dy * dy;

                // Separation
                if dist_sq < self.separation_radius * self.separation_radius && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    sep_x -= dx / dist;
                    sep_y -= dy / dist;
                    sep_count += 1;
                }

                // Alignment
                if dist_sq < self.alignment_radius * self.alignment_radius {
                    align_x += ovx;
                    align_y += ovy;
                    align_count += 1;
                }

                // Cohesion
                if dist_sq < self.cohesion_radius * self.cohesion_radius {
                    coh_x += ox;
                    coh_y += oy;
                    coh_count += 1;
                }
            }

            let mut ax = 0.0;
            let mut ay = 0.0;

            // Apply separation
            if sep_count > 0 {
                sep_x /= sep_count as f32;
                sep_y /= sep_count as f32;
                let mag = (sep_x * sep_x + sep_y * sep_y).sqrt();
                if mag > 0.0 {
                    sep_x = (sep_x / mag) * self.max_speed - boid.vx;
                    sep_y = (sep_y / mag) * self.max_speed - boid.vy;
                    ax += sep_x * self.separation_strength;
                    ay += sep_y * self.separation_strength;
                }
            }

            // Apply alignment
            if align_count > 0 {
                align_x /= align_count as f32;
                align_y /= align_count as f32;
                let mag = (align_x * align_x + align_y * align_y).sqrt();
                if mag > 0.0 {
                    align_x = (align_x / mag) * self.max_speed - boid.vx;
                    align_y = (align_y / mag) * self.max_speed - boid.vy;
                    ax += align_x * self.alignment_strength;
                    ay += align_y * self.alignment_strength;
                }
            }

            // Apply cohesion
            if coh_count > 0 {
                coh_x /= coh_count as f32;
                coh_y /= coh_count as f32;
                coh_x -= boid.x;
                coh_y -= boid.y;
                let mag = (coh_x * coh_x + coh_y * coh_y).sqrt();
                if mag > 0.0 {
                    coh_x = (coh_x / mag) * self.max_speed - boid.vx;
                    coh_y = (coh_y / mag) * self.max_speed - boid.vy;
                    ax += coh_x * self.cohesion_strength;
                    ay += coh_y * self.cohesion_strength;
                }
            }

            // Apply flock mode modifiers
            match self.flock_mode {
                FlockMode::Chaotic => {
                    let mut rng = rand::thread_rng();
                    ax += rng.gen_range(-1.0..1.0);
                    ay += rng.gen_range(-1.0..1.0);
                }
                FlockMode::Ordered => {
                    ax *= 2.0;
                    ay *= 2.0;
                }
                FlockMode::Swirl => {
                    let cx = width as f32 / 2.0;
                    let cy = height as f32 / 2.0;
                    let dx = boid.x - cx;
                    let dy = boid.y - cy;
                    ax += -dy * 0.05;
                    ay += dx * 0.05;
                }
                FlockMode::Normal => {}
            }

            // Avoid predators
            for &(px, py) in &predator_positions {
                let dx = px - boid.x;
                let dy = py - boid.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < self.predator_radius * self.predator_radius && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    ax -= (dx / dist) * self.predator_strength;
                    ay -= (dy / dist) * self.predator_strength;
                }
            }

            // Avoid mouse
            if self.avoid_mouse {
                let dx = self.mouse_x - boid.x;
                let dy = self.mouse_y - boid.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < self.mouse_avoidance_radius * self.mouse_avoidance_radius && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    ax -= (dx / dist) * self.mouse_avoidance_strength;
                    ay -= (dy / dist) * self.mouse_avoidance_strength;
                }
            }

            // Limit force
            let force_mag = (ax * ax + ay * ay).sqrt();
            if force_mag > self.max_force {
                ax = (ax / force_mag) * self.max_force;
                ay = (ay / force_mag) * self.max_force;
            }

            // Update velocity
            boid.vx += ax;
            boid.vy += ay;

            // Limit speed
            let speed = (boid.vx * boid.vx + boid.vy * boid.vy).sqrt();
            if speed > self.max_speed {
                boid.vx = (boid.vx / speed) * self.max_speed;
                boid.vy = (boid.vy / speed) * self.max_speed;
            }

            // Update position
            boid.x += boid.vx;
            boid.y += boid.vy;

            // Handle edges
            if self.wrap_edges {
                if boid.x < 0.0 {
                    boid.x += width as f32;
                }
                if boid.x >= width as f32 {
                    boid.x -= width as f32;
                }
                if boid.y < 0.0 {
                    boid.y += height as f32;
                }
                if boid.y >= height as f32 {
                    boid.y -= height as f32;
                }
            } else {
                if boid.x < 0.0 || boid.x >= width as f32 {
                    boid.vx *= -1.0;
                    boid.x = boid.x.clamp(0.0, width as f32 - 1.0);
                }
                if boid.y < 0.0 || boid.y >= height as f32 {
                    boid.vy *= -1.0;
                    boid.y = boid.y.clamp(0.0, height as f32 - 1.0);
                }
            }

            // Update color
            let t = if self.color_by_velocity {
                let speed = (boid.vx * boid.vx + boid.vy * boid.vy).sqrt();
                (speed / self.max_speed).min(1.0)
            } else if self.color_by_direction {
                let angle = boid.vy.atan2(boid.vx);
                (angle + std::f32::consts::PI) / std::f32::consts::TAU
            } else if self.color_by_neighbors {
                ((sep_count + align_count + coh_count) as f32 / (self.boid_count as f32 * 0.5)).min(1.0)
            } else {
                i as f32 / self.boid_count as f32
            };

            let mut color = color_scheme.map(t, true);

            // Apply color adjustments inline
            let (h, s, v) = Color::rgb_to_hsv_static(color);
            let new_h = (h + hue_shift * 360.0) % 360.0;
            let new_s = (s * saturation).clamp(0.0, 1.0);
            let new_v = (v * brightness).clamp(0.0, 1.0);
            color = Color::from_hsv(new_h, new_s, new_v);

            boid.color = color;
        }

        // Update predators (simple chase behavior)
        for predator in &mut self.predators {
            // Find nearest boid
            let mut nearest_dist = f32::MAX;
            let mut nearest_x = 0.0;
            let mut nearest_y = 0.0;

            for &(bx, by, _, _) in &boid_positions {
                let dx = bx - predator.x;
                let dy = by - predator.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < nearest_dist {
                    nearest_dist = dist_sq;
                    nearest_x = bx;
                    nearest_y = by;
                }
            }

            // Chase nearest boid
            if nearest_dist < f32::MAX {
                let dx = nearest_x - predator.x;
                let dy = nearest_y - predator.y;
                let dist = nearest_dist.sqrt();

                if dist > 0.0 {
                    predator.vx = (dx / dist) * self.max_speed * 0.8;
                    predator.vy = (dy / dist) * self.max_speed * 0.8;
                }
            }

            predator.x += predator.vx;
            predator.y += predator.vy;

            // Wrap edges
            if predator.x < 0.0 {
                predator.x += width as f32;
            }
            if predator.x >= width as f32 {
                predator.x -= width as f32;
            }
            if predator.y < 0.0 {
                predator.y += height as f32;
            }
            if predator.y >= height as f32 {
                predator.y -= height as f32;
            }
        }
    }

    #[allow(dead_code)]
    fn apply_color_adjustments(&self, color: Color) -> Color {
        let (h, s, v) = self.rgb_to_hsv(color);
        let new_h = (h + self.hue_shift * 360.0) % 360.0;
        let new_s = (s * self.saturation).clamp(0.0, 1.0);
        let new_v = (v * self.brightness).clamp(0.0, 1.0);

        Color::from_hsv(new_h, new_s, new_v)
    }

    #[allow(dead_code)]
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

    fn draw_triangle(&self, pixels: &mut [Color], width: usize, height: usize,
                     x: f32, y: f32, vx: f32, vy: f32, size: f32, color: Color) {
        let angle = vy.atan2(vx);

        // Triangle points
        let p1_x = x + size * angle.cos();
        let p1_y = y + size * angle.sin();

        let p2_x = x + (size * 0.5) * (angle + 2.5).cos();
        let p2_y = y + (size * 0.5) * (angle + 2.5).sin();

        let p3_x = x + (size * 0.5) * (angle - 2.5).cos();
        let p3_y = y + (size * 0.5) * (angle - 2.5).sin();

        // Draw simple triangle (just the vertices for now)
        self.draw_circle(pixels, width, height, p1_x, p1_y, 1.5, color);
        self.draw_circle(pixels, width, height, p2_x, p2_y, 1.5, color);
        self.draw_circle(pixels, width, height, p3_x, p3_y, 1.5, color);
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
                    pixels[y * width + x] = color;
                }
            }
        }
    }

    fn draw_line(&self, pixels: &mut [Color], width: usize, height: usize,
                 x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                pixels[y as usize * width + x as usize] = color;
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl Simulation2D for Boids {
    fn name(&self) -> &str {
        "Boids Flocking"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        // Apply trail fade
        if self.background_trails {
            for pixel in pixels.iter_mut() {
                let fade = (255.0 * self.trail_fade) as u8;
                *pixel = Color::from_rgb(
                    ((pixel.r as u16 * fade as u16) / 255) as u8,
                    ((pixel.g as u16 * fade as u16) / 255) as u8,
                    ((pixel.b as u16 * fade as u16) / 255) as u8,
                );
            }
        }

        // Draw perception radius (if enabled)
        if self.show_perception_radius && !self.boids.is_empty() {
            let boid = &self.boids[0];
            let radius_color = Color::from_rgb(50, 50, 50);

            // Draw cohesion radius
            for angle in 0..360 {
                let rad = (angle as f32).to_radians();
                let x = boid.x + self.cohesion_radius * rad.cos();
                let y = boid.y + self.cohesion_radius * rad.sin();
                if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
                    pixels[y as usize * width + x as usize] = radius_color;
                }
            }
        }

        // Draw boids
        for boid in &self.boids {
            if boid.x < 0.0 || boid.x >= width as f32 ||
               boid.y < 0.0 || boid.y >= height as f32 {
                continue;
            }

            if self.draw_triangles {
                self.draw_triangle(&mut pixels, width, height, boid.x, boid.y, boid.vx, boid.vy, boid.size, boid.color);
            } else {
                self.draw_circle(&mut pixels, width, height, boid.x, boid.y, boid.size, boid.color);
            }

            // Draw velocity line
            if self.show_velocity_lines {
                let end_x = boid.x + boid.vx * 3.0;
                let end_y = boid.y + boid.vy * 3.0;
                self.draw_line(&mut pixels, width, height,
                    boid.x as i32, boid.y as i32,
                    end_x as i32, end_y as i32,
                    boid.color);
            }
        }

        // Draw predators
        for predator in &self.predators {
            if predator.x >= 0.0 && predator.x < width as f32 &&
               predator.y >= 0.0 && predator.y < height as f32 {
                self.draw_circle(&mut pixels, width, height, predator.x, predator.y, predator.size, Color::RED);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Boids Flocking");

        egui::CollapsingHeader::new("🐦 Boid Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.boid_count, 10..=500)
                    .text("Boid Count")).changed() {
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.max_speed, 1.0..=10.0)
                    .text("Max Speed")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.max_force, 0.1..=2.0)
                    .text("Max Force")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.boid_size_min, 1.0..=10.0)
                    .text("Min Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.boid_size_max, 1.0..=15.0)
                    .text("Max Size")).changed();
            });

        egui::CollapsingHeader::new("🎯 Flocking Behavior")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.separation_radius, 5.0..=100.0)
                    .text("Separation Radius")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.alignment_radius, 10.0..=150.0)
                    .text("Alignment Radius")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.cohesion_radius, 10.0..=150.0)
                    .text("Cohesion Radius")).changed();

                ui.separator();

                changed |= ui.add(egui::Slider::new(&mut self.separation_strength, 0.0..=5.0)
                    .text("Separation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.alignment_strength, 0.0..=5.0)
                    .text("Alignment")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.cohesion_strength, 0.0..=5.0)
                    .text("Cohesion")).changed();

                ui.separator();

                egui::ComboBox::from_label("Flock Mode")
                    .selected_text(self.flock_mode.name())
                    .show_ui(ui, |ui| {
                        for mode in FlockMode::all() {
                            if ui.selectable_value(&mut self.flock_mode, mode, mode.name()).clicked() {
                                changed = true;
                            }
                        }
                    });
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
                changed |= ui.checkbox(&mut self.color_by_neighbors, "Color by Neighbors").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();
            });

        egui::CollapsingHeader::new("🖌 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.draw_triangles, "Draw as Triangles").changed();
                changed |= ui.checkbox(&mut self.show_velocity_lines, "Show Velocity").changed();
                changed |= ui.checkbox(&mut self.show_perception_radius, "Show Perception Radius").changed();
                changed |= ui.checkbox(&mut self.background_trails, "Show Trails").changed();

                if self.background_trails {
                    changed |= ui.add(egui::Slider::new(&mut self.trail_fade, 0.0..=1.0)
                        .text("Trail Fade")).changed();
                }

                changed |= ui.checkbox(&mut self.wrap_edges, "Wrap Edges").changed();
            });

        egui::CollapsingHeader::new("🎮 Interaction")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.avoid_mouse, "Avoid Mouse").changed();

                if self.avoid_mouse {
                    changed |= ui.add(egui::Slider::new(&mut self.mouse_avoidance_radius, 20.0..=200.0)
                        .text("Avoidance Radius")).changed();

                    changed |= ui.add(egui::Slider::new(&mut self.mouse_avoidance_strength, 0.5..=5.0)
                        .text("Avoidance Strength")).changed();
                }
            });

        egui::CollapsingHeader::new("👹 Predators")
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.predator_count, 0..=10)
                    .text("Predator Count")).changed() {
                    changed = true;
                }

                if self.predator_count > 0 {
                    changed |= ui.add(egui::Slider::new(&mut self.predator_radius, 50.0..=200.0)
                        .text("Fear Radius")).changed();

                    changed |= ui.add(egui::Slider::new(&mut self.predator_strength, 1.0..=10.0)
                        .text("Fear Strength")).changed();
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic Flock").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Tight Formation").clicked() {
                    self.cohesion_strength = 3.0;
                    self.alignment_strength = 2.0;
                    self.separation_strength = 0.5;
                    changed = true;
                }
                if ui.button("Chaotic Swarm").clicked() {
                    self.flock_mode = FlockMode::Chaotic;
                    self.boid_count = 300;
                    self.max_speed = 6.0;
                    changed = true;
                }
                if ui.button("Rainbow School").clicked() {
                    self.color_scheme = ColorScheme::Rainbow;
                    self.color_by_direction = true;
                    self.boid_count = 200;
                    changed = true;
                }
            });

        // Update boid positions
        let dt = ui.input(|i| i.stable_dt);

        // Reinitialize if count changed
        let current_count = self.boids.len();
        let current_predator_count = self.predators.len();
        if current_count != self.boid_count || current_predator_count != self.predator_count {
            let size = ui.available_size();
            self.init_boids(size.x as usize, size.y as usize);
        }

        // Update positions
        let size = ui.available_size();
        self.update_boids(size.x.max(800.0) as usize, size.y.max(600.0) as usize, dt);
        changed = true;

        changed
    }
}
