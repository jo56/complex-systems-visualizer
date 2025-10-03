use crate::{Color, ColorScheme, Simulation2D};

pub struct DoublePendulum {
    pub length1: f32,
    pub length2: f32,
    pub mass1: f32,
    pub mass2: f32,
    pub gravity: f32,
    pub damping: f32,
    pub trace_length: usize,
    pub show_trace: bool,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    angle1: f32,
    angle2: f32,
    velocity1: f32,
    velocity2: f32,
    trace: Vec<(f32, f32)>,
}

impl Default for DoublePendulum {
    fn default() -> Self {
        Self {
            length1: 0.2,  // Now as ratio of canvas size (0.0-1.0)
            length2: 0.2,
            mass1: 10.0,
            mass2: 10.0,
            gravity: 1.0,
            damping: 0.9999,
            trace_length: 500,
            show_trace: true,
            color_scheme: ColorScheme::Rainbow,
            scale: 1.0,  // Overall scale multiplier
            angle1: std::f32::consts::PI / 2.0,
            angle2: std::f32::consts::PI / 2.0,
            velocity1: 0.0,
            velocity2: 0.0,
            trace: Vec::new(),
        }
    }
}

impl DoublePendulum {
    pub fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, dt: f32, canvas_scale: f32) {
        let g = self.gravity;
        let m1 = self.mass1;
        let m2 = self.mass2;
        // Use actual pixel lengths for physics
        let l1 = self.length1 * canvas_scale * self.scale;
        let l2 = self.length2 * canvas_scale * self.scale;
        let a1 = self.angle1;
        let a2 = self.angle2;
        let v1 = self.velocity1;
        let v2 = self.velocity2;

        // Double pendulum equations (Lagrangian mechanics)
        let num1 = -g * (2.0 * m1 + m2) * a1.sin()
                   - m2 * g * (a1 - 2.0 * a2).sin()
                   - 2.0 * (a1 - a2).sin() * m2 * (v2 * v2 * l2 + v1 * v1 * l1 * (a1 - a2).cos());
        let den1 = l1 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos());
        let acc1 = num1 / den1;

        let num2 = 2.0 * (a1 - a2).sin()
                   * (v1 * v1 * l1 * (m1 + m2) + g * (m1 + m2) * a1.cos() + v2 * v2 * l2 * m2 * (a1 - a2).cos());
        let den2 = l2 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos());
        let acc2 = num2 / den2;

        self.velocity1 += acc1 * dt;
        self.velocity2 += acc2 * dt;

        // Apply damping
        self.velocity1 *= self.damping;
        self.velocity2 *= self.damping;

        self.angle1 += self.velocity1 * dt;
        self.angle2 += self.velocity2 * dt;

        // Calculate second bob position for trace
        let x2 = l1 * self.angle1.sin() + l2 * self.angle2.sin();
        let y2 = l1 * self.angle1.cos() + l2 * self.angle2.cos();

        self.trace.push((x2, y2));
        if self.trace.len() > self.trace_length {
            self.trace.remove(0);
        }
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

impl Simulation2D for DoublePendulum {
    fn name(&self) -> &str {
        "Double Pendulum"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let cx = width as f32 / 2.0;
        let cy = height as f32 / 4.0;

        // Calculate canvas scale - use the smaller dimension to ensure it fits
        let canvas_scale = width.min(height) as f32;

        // Actual lengths in pixels (ratio * canvas_scale * user scale)
        let len1_px = self.length1 * canvas_scale * self.scale;
        let len2_px = self.length2 * canvas_scale * self.scale;

        // Draw trace
        if self.show_trace {
            for (i, &(x, y)) in self.trace.iter().enumerate() {
                let px = cx + x;
                let py = cy + y;

                if px >= 0.0 && px < width as f32 && py >= 0.0 && py < height as f32 {
                    let t = i as f32 / self.trace.len() as f32;
                    let color = self.color_scheme.map(t, true);
                    self.draw_circle(&mut pixels, width, height, px, py, 2.0, color);
                }
            }
        }

        // Calculate bob positions
        let x1 = cx + len1_px * self.angle1.sin();
        let y1 = cy + len1_px * self.angle1.cos();

        let x2 = x1 + len2_px * self.angle2.sin();
        let y2 = y1 + len2_px * self.angle2.cos();

        // Draw rods
        self.draw_line(&mut pixels, width, height,
            cx as i32, cy as i32, x1 as i32, y1 as i32, Color::WHITE);
        self.draw_line(&mut pixels, width, height,
            x1 as i32, y1 as i32, x2 as i32, y2 as i32, Color::WHITE);

        // Draw bobs
        let bob1_radius = (self.mass1 * 2.0).sqrt();
        let bob2_radius = (self.mass2 * 2.0).sqrt();

        self.draw_circle(&mut pixels, width, height, cx, cy, 5.0, Color::from_rgb(100, 100, 100));
        self.draw_circle(&mut pixels, width, height, x1, y1, bob1_radius, Color::from_rgb(255, 100, 100));
        self.draw_circle(&mut pixels, width, height, x2, y2, bob2_radius, Color::from_rgb(100, 100, 255));

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Double Pendulum");

        egui::CollapsingHeader::new("⚙ Physical Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.length1, 0.05..=0.4)
                    .text("Length 1 (ratio)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.length2, 0.05..=0.4)
                    .text("Length 2 (ratio)")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.mass1, 1.0..=50.0)
                    .text("Mass 1")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.mass2, 1.0..=50.0)
                    .text("Mass 2")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.gravity, 0.1..=3.0)
                    .text("Gravity")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.damping, 0.99..=1.0)
                    .text("Damping")).changed();
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_trace, "Show Trace").changed();

                if self.show_trace {
                    changed |= ui.add(egui::Slider::new(&mut self.trace_length, 10..=2000)
                        .text("Trace Length")).changed();
                }

                changed |= ui.add(egui::Slider::new(&mut self.scale, 0.3..=1.5)
                    .text("Scale")).changed();

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

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic").clicked() {
                    self.angle1 = std::f32::consts::PI / 2.0;
                    self.angle2 = std::f32::consts::PI / 2.0;
                    self.velocity1 = 0.0;
                    self.velocity2 = 0.0;
                    self.trace.clear();
                    changed = true;
                }
                if ui.button("Chaotic Start").clicked() {
                    self.angle1 = std::f32::consts::PI / 2.0 + 0.1;
                    self.angle2 = std::f32::consts::PI / 2.0;
                    self.velocity1 = 0.0;
                    self.velocity2 = 0.0;
                    self.trace.clear();
                    changed = true;
                }
                if ui.button("High Energy").clicked() {
                    self.angle1 = std::f32::consts::PI;
                    self.angle2 = 0.0;
                    self.velocity1 = 0.2;
                    self.velocity2 = 0.1;
                    self.trace.clear();
                    changed = true;
                }
            });

        if ui.button("🔄 Reset").clicked() {
            self.angle1 = std::f32::consts::PI / 2.0;
            self.angle2 = std::f32::consts::PI / 2.0;
            self.velocity1 = 0.0;
            self.velocity2 = 0.0;
            self.trace.clear();
        }

        // Update simulation
        let dt = ui.input(|i| i.stable_dt);
        let size = ui.available_size();
        let canvas_scale = size.x.min(size.y);
        for _ in 0..3 {
            self.update(dt * 10.0, canvas_scale);
        }
        changed = true;

        changed
    }
}
