use crate::{Color, ColorScheme, Simulation2D};

pub struct Phyllotaxis {
    pub dot_count: usize,
    pub angle: f32,  // Golden angle by default (137.508°)
    pub scale: f32,
    pub c_value: f32,  // Scaling factor for spiral
    pub dot_size_min: f32,
    pub dot_size_max: f32,
    pub size_by_age: bool,
    pub color_scheme: ColorScheme,
    pub color_by_angle: bool,
    pub color_by_distance: bool,
    pub rotation: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub animate_rotation: bool,
    pub animate_growth: bool,
    pub animation_speed: f32,
    pub show_spirals: bool,
    pub spiral_count: usize,
    pub fade_by_age: bool,
    animation_time: f32,
}

impl Default for Phyllotaxis {
    fn default() -> Self {
        Self {
            dot_count: 500,
            angle: 137.508,  // Golden angle
            scale: 1.0,
            c_value: 4.0,
            dot_size_min: 2.0,
            dot_size_max: 8.0,
            size_by_age: true,
            color_scheme: ColorScheme::Rainbow,
            color_by_angle: false,
            color_by_distance: true,
            rotation: 0.0,
            center_x: 0.5,
            center_y: 0.5,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            animate_rotation: false,
            animate_growth: false,
            animation_speed: 1.0,
            show_spirals: false,
            spiral_count: 13,
            fade_by_age: false,
            animation_time: 0.0,
        }
    }
}

impl Phyllotaxis {
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
                    pixels[y * width + x] = color;
                }
            }
        }
    }
}

impl Simulation2D for Phyllotaxis {
    fn name(&self) -> &str {
        "Phyllotaxis Pattern"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let size = width.min(height) as f32;
        let cx = width as f32 * self.center_x;
        let cy = height as f32 * self.center_y;

        let max_dots = if self.animate_growth {
            ((self.dot_count as f32 * (self.animation_time * self.animation_speed * 0.5).sin().abs()) as usize).max(10)
        } else {
            self.dot_count
        };

        let angle_rad = (self.angle + if self.animate_rotation {
            self.animation_time * self.animation_speed * 10.0
        } else {
            self.rotation
        }).to_radians();

        for n in 0..max_dots {
            let a = n as f32 * angle_rad;
            let r = self.c_value * (n as f32).sqrt() * self.scale;

            let x = cx + r * a.cos();
            let y = cy + r * a.sin();

            if x < 0.0 || x >= width as f32 || y < 0.0 || y >= height as f32 {
                continue;
            }

            let t = if self.color_by_angle {
                (a % (2.0 * std::f32::consts::PI)) / (2.0 * std::f32::consts::PI)
            } else if self.color_by_distance {
                (r / (size * 0.5 * self.scale)).min(1.0)
            } else {
                n as f32 / max_dots as f32
            };

            let mut color = self.color_scheme.map(t, true);
            color = self.apply_color_adjustments(color);

            if self.fade_by_age {
                let age_factor = n as f32 / max_dots as f32;
                // Apply alpha blending would go here if supported
                let fade = (age_factor * 255.0) as u8;
                color = Color::from_rgb(
                    ((color.r as u16 * fade as u16) / 255) as u8,
                    ((color.g as u16 * fade as u16) / 255) as u8,
                    ((color.b as u16 * fade as u16) / 255) as u8,
                );
            }

            let dot_size = if self.size_by_age {
                let age_t = n as f32 / max_dots as f32;
                self.dot_size_min + (self.dot_size_max - self.dot_size_min) * age_t
            } else {
                self.dot_size_max
            };

            self.draw_circle(&mut pixels, width, height, x, y, dot_size, color);
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Phyllotaxis Pattern");

        egui::CollapsingHeader::new("⚙ Pattern Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.dot_count, 10..=2000)
                    .text("Dot Count")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.angle, 120.0..=150.0)
                    .text("Angle (°)")).changed();

                ui.label(format!("Golden Angle: 137.508°"));
                if ui.button("Set Golden Angle").clicked() {
                    self.angle = 137.508;
                    changed = true;
                }

                changed |= ui.add(egui::Slider::new(&mut self.c_value, 1.0..=10.0)
                    .text("Spiral Tightness")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.scale, 0.1..=2.0)
                    .text("Scale")).changed();
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

                changed |= ui.checkbox(&mut self.color_by_distance, "Color by Distance").changed();
                changed |= ui.checkbox(&mut self.color_by_angle, "Color by Angle").changed();
                changed |= ui.checkbox(&mut self.fade_by_age, "Fade by Age").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();
            });

        egui::CollapsingHeader::new("⚫ Dot Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.dot_size_min, 0.5..=10.0)
                    .text("Min Size")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.dot_size_max, 0.5..=20.0)
                    .text("Max Size")).changed();

                changed |= ui.checkbox(&mut self.size_by_age, "Size by Age").changed();
            });

        egui::CollapsingHeader::new("🎬 Animation")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.animate_rotation, "Rotate").changed();
                changed |= ui.checkbox(&mut self.animate_growth, "Pulse Growth").changed();

                if self.animate_rotation || self.animate_growth {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("📍 Position & Rotation")
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.center_x, 0.0..=1.0)
                    .text("Center X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_y, 0.0..=1.0)
                    .text("Center Y")).changed();

                if !self.animate_rotation {
                    changed |= ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0)
                        .text("Rotation")).changed();
                }

                if ui.button("Reset Position").clicked() {
                    self.center_x = 0.5;
                    self.center_y = 0.5;
                    self.rotation = 0.0;
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic Sunflower").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Rainbow Spiral").clicked() {
                    self.color_scheme = ColorScheme::Rainbow;
                    self.dot_count = 1000;
                    self.color_by_distance = true;
                    changed = true;
                }
                if ui.button("Fire Flower").clicked() {
                    self.color_scheme = ColorScheme::Fire;
                    self.dot_count = 800;
                    self.c_value = 5.0;
                    changed = true;
                }
                if ui.button("Galaxy Bloom").clicked() {
                    self.color_scheme = ColorScheme::Galaxy;
                    self.dot_count = 1500;
                    self.animate_rotation = true;
                    changed = true;
                }
            });

        // Handle animation
        if self.animate_rotation || self.animate_growth {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt;
            changed = true;
        }

        changed
    }
}
