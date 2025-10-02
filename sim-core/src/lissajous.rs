use crate::{Color, ColorScheme, Simulation2D};

pub struct LissajousCurves {
    pub freq_x: f32,
    pub freq_y: f32,
    pub phase: f32,
    pub amplitude_x: f32,
    pub amplitude_y: f32,
    pub line_width: f32,
    pub point_count: usize,
    pub color_scheme: ColorScheme,
    pub animate_phase: bool,
    pub animate_frequency: bool,
    pub animation_speed: f32,
    pub show_points: bool,
    pub trail_fade: bool,
    animation_time: f32,
}

impl Default for LissajousCurves {
    fn default() -> Self {
        Self {
            freq_x: 3.0,
            freq_y: 4.0,
            phase: std::f32::consts::PI / 2.0,
            amplitude_x: 300.0,
            amplitude_y: 300.0,
            line_width: 2.0,
            point_count: 1000,
            color_scheme: ColorScheme::Rainbow,
            animate_phase: true,
            animate_frequency: false,
            animation_speed: 1.0,
            show_points: false,
            trail_fade: false,
            animation_time: 0.0,
        }
    }
}

impl LissajousCurves {
    pub fn new() -> Self {
        Self::default()
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

impl Simulation2D for LissajousCurves {
    fn name(&self) -> &str {
        "Lissajous Curves"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        let phase = if self.animate_phase {
            self.phase + self.animation_time * self.animation_speed
        } else {
            self.phase
        };

        let freq_x = if self.animate_frequency {
            self.freq_x + (self.animation_time * self.animation_speed * 0.5).sin()
        } else {
            self.freq_x
        };

        let freq_y = if self.animate_frequency {
            self.freq_y + (self.animation_time * self.animation_speed * 0.3).cos()
        } else {
            self.freq_y
        };

        let mut points = Vec::new();

        for i in 0..self.point_count {
            let t = (i as f32 / self.point_count as f32) * std::f32::consts::TAU;

            let x = cx + self.amplitude_x * (freq_x * t).sin();
            let y = cy + self.amplitude_y * (freq_y * t + phase).sin();

            points.push((x, y));
        }

        // Draw lines
        for i in 0..points.len() - 1 {
            let (x0, y0) = points[i];
            let (x1, y1) = points[i + 1];

            let t = i as f32 / points.len() as f32;
            let color = self.color_scheme.map(t, true);

            for w in 0..(self.line_width as i32) {
                self.draw_line(&mut pixels, width, height,
                    x0 as i32 + w, y0 as i32,
                    x1 as i32 + w, y1 as i32,
                    color);
                self.draw_line(&mut pixels, width, height,
                    x0 as i32, y0 as i32 + w,
                    x1 as i32, y1 as i32 + w,
                    color);
            }
        }

        // Draw points
        if self.show_points {
            for (i, &(x, y)) in points.iter().enumerate() {
                let t = i as f32 / points.len() as f32;
                let color = self.color_scheme.map(t, true);
                self.draw_circle(&mut pixels, width, height, x, y, 3.0, color);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Lissajous Curves");

        egui::CollapsingHeader::new("⚙ Curve Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.freq_x, 1.0..=10.0)
                    .text("Frequency X")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.freq_y, 1.0..=10.0)
                    .text("Frequency Y")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.phase, 0.0..=std::f32::consts::TAU)
                    .text("Phase")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.amplitude_x, 50.0..=400.0)
                    .text("Amplitude X")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.amplitude_y, 50.0..=400.0)
                    .text("Amplitude Y")).changed();
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.point_count, 100..=5000)
                    .text("Point Count")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.line_width, 1.0..=5.0)
                    .text("Line Width")).changed();

                changed |= ui.checkbox(&mut self.show_points, "Show Points").changed();

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

        egui::CollapsingHeader::new("🎬 Animation")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.animate_phase, "Animate Phase").changed();
                changed |= ui.checkbox(&mut self.animate_frequency, "Animate Frequency").changed();

                if self.animate_phase || self.animate_frequency {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Circle").clicked() {
                    self.freq_x = 1.0;
                    self.freq_y = 1.0;
                    self.phase = std::f32::consts::PI / 2.0;
                    changed = true;
                }
                if ui.button("Figure Eight").clicked() {
                    self.freq_x = 1.0;
                    self.freq_y = 2.0;
                    self.phase = std::f32::consts::PI / 2.0;
                    changed = true;
                }
                if ui.button("Flower").clicked() {
                    self.freq_x = 3.0;
                    self.freq_y = 4.0;
                    self.phase = std::f32::consts::PI / 2.0;
                    changed = true;
                }
                if ui.button("Star").clicked() {
                    self.freq_x = 5.0;
                    self.freq_y = 4.0;
                    self.phase = 0.0;
                    changed = true;
                }
                if ui.button("Complex").clicked() {
                    self.freq_x = 7.0;
                    self.freq_y = 9.0;
                    self.phase = std::f32::consts::PI / 4.0;
                    changed = true;
                }
            });

        // Update animation
        if self.animate_phase || self.animate_frequency {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt;
            changed = true;
        }

        changed
    }
}
