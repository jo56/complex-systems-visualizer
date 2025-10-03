use crate::{Color, ColorScheme, Simulation2D};

// De Jong (Peter de Jong) Attractor
pub struct DeJongAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub point_count: usize,
    pub point_size: f32,
    pub zoom: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub color_scheme: ColorScheme,
    pub color_by_position: bool,
    pub color_by_iteration: bool,
    pub color_by_distance: bool,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub fade_by_age: bool,
    pub animate_params: bool,
    pub animation_speed: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub point_glow: bool,
    pub background_fade: f32,
    animation_time: f32,
}

impl Default for DeJongAttractor {
    fn default() -> Self {
        Self {
            a: -2.0,
            b: -2.0,
            c: -1.2,
            d: 2.0,
            point_count: 10000,
            point_size: 1.0,
            zoom: 200.0,
            center_x: 0.5,
            center_y: 0.5,
            color_scheme: ColorScheme::Rainbow,
            color_by_position: false,
            color_by_iteration: true,
            color_by_distance: false,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            fade_by_age: false,
            animate_params: false,
            animation_speed: 1.0,
            start_x: 0.0,
            start_y: 0.0,
            point_glow: false,
            background_fade: 0.0,
            animation_time: 0.0,
        }
    }
}

// Clifford Attractor
pub struct CliffordAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub point_count: usize,
    pub point_size: f32,
    pub zoom: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub color_scheme: ColorScheme,
    pub color_by_position: bool,
    pub color_by_iteration: bool,
    pub color_by_angle: bool,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub fade_by_age: bool,
    pub animate_params: bool,
    pub animation_speed: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub point_glow: bool,
    pub background_fade: f32,
    animation_time: f32,
}

impl Default for CliffordAttractor {
    fn default() -> Self {
        Self {
            a: -1.4,
            b: 1.6,
            c: 1.0,
            d: 0.7,
            point_count: 10000,
            point_size: 1.0,
            zoom: 150.0,
            center_x: 0.5,
            center_y: 0.5,
            color_scheme: ColorScheme::Plasma,
            color_by_position: false,
            color_by_iteration: true,
            color_by_angle: false,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            fade_by_age: false,
            animate_params: false,
            animation_speed: 1.0,
            start_x: 0.0,
            start_y: 0.0,
            point_glow: false,
            background_fade: 0.0,
            animation_time: 0.0,
        }
    }
}

impl DeJongAttractor {
    pub fn new() -> Self {
        Self::default()
    }

    fn apply_color_adjustments(&self, color: Color) -> Color {
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
                    if self.point_glow {
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

impl CliffordAttractor {
    pub fn new() -> Self {
        Self::default()
    }

    fn apply_color_adjustments(&self, color: Color) -> Color {
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
                    if self.point_glow {
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

impl Simulation2D for DeJongAttractor {
    fn name(&self) -> &str {
        "De Jong Attractor"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let cx = width as f32 * self.center_x;
        let cy = height as f32 * self.center_y;

        let mut x = self.start_x;
        let mut y = self.start_y;

        // Animated parameters
        let (a, b, c, d) = if self.animate_params {
            let t = self.animation_time * self.animation_speed;
            (
                self.a + (t * 0.5).sin() * 0.5,
                self.b + (t * 0.7).cos() * 0.5,
                self.c + (t * 0.3).sin() * 0.3,
                self.d + (t * 0.6).cos() * 0.3,
            )
        } else {
            (self.a, self.b, self.c, self.d)
        };

        for i in 0..self.point_count {
            // De Jong attractor formula: x' = sin(a*y) - cos(b*x), y' = sin(c*x) - cos(d*y)
            let x_new = (a * y).sin() - (b * x).cos();
            let y_new = (c * x).sin() - (d * y).cos();

            x = x_new;
            y = y_new;

            let px = cx + x * self.zoom;
            let py = cy + y * self.zoom;

            if px >= 0.0 && px < width as f32 && py >= 0.0 && py < height as f32 {
                let t = if self.color_by_iteration {
                    i as f32 / self.point_count as f32
                } else if self.color_by_position {
                    ((x + 2.0) / 4.0).clamp(0.0, 1.0)
                } else if self.color_by_distance {
                    let dist = (x * x + y * y).sqrt();
                    (dist / 2.0).clamp(0.0, 1.0)
                } else {
                    0.5
                };

                let mut color = self.color_scheme.map(t, true);
                color = self.apply_color_adjustments(color);

                if self.fade_by_age {
                    let age_t = i as f32 / self.point_count as f32;
                    color = Color::from_rgb(
                        (color.r as f32 * age_t) as u8,
                        (color.g as f32 * age_t) as u8,
                        (color.b as f32 * age_t) as u8,
                    );
                }

                self.draw_circle(&mut pixels, width, height, px, py, self.point_size, color);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("De Jong Attractor");

        egui::CollapsingHeader::new("⚙ Attractor Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, -3.0..=3.0)
                    .text("Parameter A")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.b, -3.0..=3.0)
                    .text("Parameter B")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.c, -3.0..=3.0)
                    .text("Parameter C")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.d, -3.0..=3.0)
                    .text("Parameter D")).changed();
            });

        egui::CollapsingHeader::new("🔍 View Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.point_count, 100..=100000)
                    .text("Point Count")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.point_size, 0.5..=5.0)
                    .text("Point Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.zoom, 50.0..=500.0)
                    .text("Zoom")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_x, 0.0..=1.0)
                    .text("Center X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_y, 0.0..=1.0)
                    .text("Center Y")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.start_x, -2.0..=2.0)
                    .text("Start X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.start_y, -2.0..=2.0)
                    .text("Start Y")).changed();
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

                changed |= ui.checkbox(&mut self.color_by_iteration, "Color by Iteration").changed();
                changed |= ui.checkbox(&mut self.color_by_position, "Color by Position").changed();
                changed |= ui.checkbox(&mut self.color_by_distance, "Color by Distance").changed();
                changed |= ui.checkbox(&mut self.fade_by_age, "Fade by Age").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();
            });

        egui::CollapsingHeader::new("✨ Visual Effects")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.point_glow, "Point Glow").changed();
                changed |= ui.add(egui::Slider::new(&mut self.background_fade, 0.0..=1.0)
                    .text("Background Fade")).changed();
            });

        egui::CollapsingHeader::new("🎬 Animation")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.animate_params, "Animate Parameters").changed();

                if self.animate_params {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Swirls").clicked() {
                    self.a = 1.4;
                    self.b = -2.3;
                    self.c = 2.4;
                    self.d = -2.1;
                    changed = true;
                }
                if ui.button("Web").clicked() {
                    self.a = -2.7;
                    self.b = -0.09;
                    self.c = -0.86;
                    self.d = -2.2;
                    self.point_count = 50000;
                    changed = true;
                }
                if ui.button("Flower").clicked() {
                    self.a = -2.01;
                    self.b = -1.8;
                    self.c = 1.79;
                    self.d = -1.93;
                    self.color_scheme = ColorScheme::Rainbow;
                    changed = true;
                }
            });

        // Handle animation
        if self.animate_params {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt;
            changed = true;
        }

        changed
    }

    fn supports_zoom(&self) -> bool {
        true
    }

    fn adjust_center(&mut self, dx: f64, dy: f64, _width: usize, _height: usize) {
        self.center_x += (dx as f32) / self.zoom;
        self.center_y += (dy as f32) / self.zoom;
    }

    fn get_zoom(&self) -> f64 {
        self.zoom as f64
    }
}

impl Simulation2D for CliffordAttractor {
    fn name(&self) -> &str {
        "Clifford Attractor"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let cx = width as f32 * self.center_x;
        let cy = height as f32 * self.center_y;

        let mut x = self.start_x;
        let mut y = self.start_y;

        // Animated parameters
        let (a, b, c, d) = if self.animate_params {
            let t = self.animation_time * self.animation_speed;
            (
                self.a + (t * 0.4).sin() * 0.3,
                self.b + (t * 0.5).cos() * 0.3,
                self.c + (t * 0.6).sin() * 0.2,
                self.d + (t * 0.7).cos() * 0.2,
            )
        } else {
            (self.a, self.b, self.c, self.d)
        };

        for i in 0..self.point_count {
            // Clifford attractor formula: x' = sin(a*y) + c*cos(a*x), y' = sin(b*x) + d*cos(b*y)
            let x_new = (a * y).sin() + c * (a * x).cos();
            let y_new = (b * x).sin() + d * (b * y).cos();

            x = x_new;
            y = y_new;

            let px = cx + x * self.zoom;
            let py = cy + y * self.zoom;

            if px >= 0.0 && px < width as f32 && py >= 0.0 && py < height as f32 {
                let t = if self.color_by_iteration {
                    i as f32 / self.point_count as f32
                } else if self.color_by_position {
                    ((x + 2.0) / 4.0).clamp(0.0, 1.0)
                } else if self.color_by_angle {
                    let angle = y.atan2(x);
                    (angle + std::f32::consts::PI) / std::f32::consts::TAU
                } else {
                    0.5
                };

                let mut color = self.color_scheme.map(t, true);
                color = self.apply_color_adjustments(color);

                if self.fade_by_age {
                    let age_t = i as f32 / self.point_count as f32;
                    color = Color::from_rgb(
                        (color.r as f32 * age_t) as u8,
                        (color.g as f32 * age_t) as u8,
                        (color.b as f32 * age_t) as u8,
                    );
                }

                self.draw_circle(&mut pixels, width, height, px, py, self.point_size, color);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Clifford Attractor");

        egui::CollapsingHeader::new("⚙ Attractor Parameters")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.a, -3.0..=3.0)
                    .text("Parameter A")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.b, -3.0..=3.0)
                    .text("Parameter B")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.c, -3.0..=3.0)
                    .text("Parameter C")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.d, -3.0..=3.0)
                    .text("Parameter D")).changed();
            });

        egui::CollapsingHeader::new("🔍 View Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.point_count, 100..=100000)
                    .text("Point Count")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.point_size, 0.5..=5.0)
                    .text("Point Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.zoom, 50.0..=500.0)
                    .text("Zoom")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_x, 0.0..=1.0)
                    .text("Center X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_y, 0.0..=1.0)
                    .text("Center Y")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.start_x, -2.0..=2.0)
                    .text("Start X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.start_y, -2.0..=2.0)
                    .text("Start Y")).changed();
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

                changed |= ui.checkbox(&mut self.color_by_iteration, "Color by Iteration").changed();
                changed |= ui.checkbox(&mut self.color_by_position, "Color by Position").changed();
                changed |= ui.checkbox(&mut self.color_by_angle, "Color by Angle").changed();
                changed |= ui.checkbox(&mut self.fade_by_age, "Fade by Age").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();
            });

        egui::CollapsingHeader::new("✨ Visual Effects")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.point_glow, "Point Glow").changed();
                changed |= ui.add(egui::Slider::new(&mut self.background_fade, 0.0..=1.0)
                    .text("Background Fade")).changed();
            });

        egui::CollapsingHeader::new("🎬 Animation")
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.animate_params, "Animate Parameters").changed();

                if self.animate_params {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Spiral Web").clicked() {
                    self.a = 1.5;
                    self.b = -1.8;
                    self.c = 1.6;
                    self.d = 0.9;
                    changed = true;
                }
                if ui.button("Butterfly").clicked() {
                    self.a = -1.7;
                    self.b = 1.3;
                    self.c = -0.1;
                    self.d = -1.2;
                    self.color_scheme = ColorScheme::Rainbow;
                    changed = true;
                }
                if ui.button("Galaxy").clicked() {
                    self.a = -1.4;
                    self.b = 1.6;
                    self.c = 1.0;
                    self.d = 0.7;
                    self.color_scheme = ColorScheme::Galaxy;
                    self.point_count = 50000;
                    changed = true;
                }
            });

        // Handle animation
        if self.animate_params {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt;
            changed = true;
        }

        changed
    }

    fn supports_zoom(&self) -> bool {
        true
    }

    fn adjust_center(&mut self, dx: f64, dy: f64, _width: usize, _height: usize) {
        self.center_x += (dx as f32) / self.zoom;
        self.center_y += (dy as f32) / self.zoom;
    }

    fn get_zoom(&self) -> f64 {
        self.zoom as f64
    }
}
