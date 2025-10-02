use crate::{Color, ColorScheme, Simulation2D};

#[derive(Clone, Copy, PartialEq)]
pub enum DrawMode {
    Lines,
    Points,
    FilledTriangles,
}

impl DrawMode {
    pub fn all() -> Vec<DrawMode> {
        vec![DrawMode::Lines, DrawMode::Points, DrawMode::FilledTriangles]
    }

    pub fn name(&self) -> &str {
        match self {
            DrawMode::Lines => "Lines",
            DrawMode::Points => "Points",
            DrawMode::FilledTriangles => "Filled Triangles",
        }
    }
}

pub struct KochSnowflake {
    pub iterations: usize,
    pub rotation: f32,
    pub scale: f32,
    pub line_width: f32,
    pub color_scheme: ColorScheme,
    pub color_by_depth: bool,
    pub draw_mode: DrawMode,
    pub animate: bool,
    pub animation_speed: f32,
    pub show_construction: bool,
    pub center_x: f32,
    pub center_y: f32,
    pub hue_shift: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub alpha: f32,
    pub invert_colors: bool,
    animation_time: f32,
}

impl Default for KochSnowflake {
    fn default() -> Self {
        Self {
            iterations: 4,
            rotation: 0.0,
            scale: 0.4,
            line_width: 1.0,
            color_scheme: ColorScheme::Ice,
            color_by_depth: true,
            draw_mode: DrawMode::Lines,
            animate: false,
            animation_speed: 1.0,
            show_construction: false,
            center_x: 0.5,
            center_y: 0.5,
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            alpha: 1.0,
            invert_colors: false,
            animation_time: 0.0,
        }
    }
}

impl KochSnowflake {
    pub fn new() -> Self {
        Self::default()
    }

    fn koch_curve(&self, p1: (f32, f32), p2: (f32, f32), depth: usize) -> Vec<(f32, f32)> {
        if depth == 0 {
            return vec![p1, p2];
        }

        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;

        let p_a = (p1.0 + dx / 3.0, p1.1 + dy / 3.0);
        let p_b = (p1.0 + 2.0 * dx / 3.0, p1.1 + 2.0 * dy / 3.0);

        let angle = 60.0_f32.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let dx_third = dx / 3.0;
        let dy_third = dy / 3.0;

        let p_c = (
            p_a.0 + dx_third * cos_a - dy_third * sin_a,
            p_a.1 + dx_third * sin_a + dy_third * cos_a,
        );

        let mut points = Vec::new();
        points.extend(self.koch_curve(p1, p_a, depth - 1));
        points.extend(self.koch_curve(p_a, p_c, depth - 1));
        points.extend(self.koch_curve(p_c, p_b, depth - 1));
        points.extend(self.koch_curve(p_b, p2, depth - 1));

        points
    }

    fn apply_color_adjustments(&self, mut color: Color) -> Color {
        // Apply hue shift, saturation, brightness
        let (h, s, v) = self.rgb_to_hsv(color);
        let new_h = (h + self.hue_shift * 360.0) % 360.0;
        let new_s = (s * self.saturation).clamp(0.0, 1.0);
        let new_v = (v * self.brightness).clamp(0.0, 1.0);

        color = Color::from_hsv(new_h, new_s, new_v);

        if self.invert_colors {
            color = Color::from_rgb(255 - color.r, 255 - color.g, 255 - color.b);
        }

        color
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
}

impl Simulation2D for KochSnowflake {
    fn name(&self) -> &str {
        "Koch Snowflake"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        let size = (width.min(height) as f32 * self.scale).max(50.0);
        let cx = width as f32 * self.center_x;
        let cy = height as f32 * self.center_y;

        let effective_rotation = if self.animate {
            self.rotation + self.animation_time * self.animation_speed
        } else {
            self.rotation
        };

        // Create three starting points for the triangle
        let angle_offset = effective_rotation;
        let points: Vec<(f32, f32)> = (0..3)
            .map(|i| {
                let angle = (i as f32 * 120.0 + angle_offset).to_radians();
                (
                    cx + size * angle.cos(),
                    cy + size * angle.sin(),
                )
            })
            .collect();

        // Generate Koch curve for each side
        for i in 0..3 {
            let p1 = points[i];
            let p2 = points[(i + 1) % 3];

            let curve_points = self.koch_curve(p1, p2, self.iterations);

            // Draw the curve
            for j in 0..curve_points.len() - 1 {
                let (x1, y1) = curve_points[j];
                let (x2, y2) = curve_points[j + 1];

                let color_t = if self.color_by_depth {
                    j as f32 / curve_points.len() as f32
                } else {
                    i as f32 / 3.0
                };

                let mut color = self.color_scheme.map(color_t, true);
                color = self.apply_color_adjustments(color);

                // Draw line using Bresenham's algorithm
                self.draw_line(&mut pixels, width, height, x1 as i32, y1 as i32, x2 as i32, y2 as i32, color);
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Koch Snowflake");

        egui::CollapsingHeader::new("⚙ Fractal Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.iterations, 0..=7)
                    .text("Iterations")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.scale, 0.1..=1.0)
                    .text("Scale")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0)
                    .text("Rotation (°)")).changed();
            });

        egui::CollapsingHeader::new("🎨 Color & Style")
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

                changed |= ui.checkbox(&mut self.color_by_depth, "Color by Depth").changed();
                changed |= ui.checkbox(&mut self.invert_colors, "Invert Colors").changed();

                changed |= ui.add(egui::Slider::new(&mut self.hue_shift, 0.0..=1.0)
                    .text("Hue Shift")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.saturation, 0.0..=2.0)
                    .text("Saturation")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.brightness, 0.0..=2.0)
                    .text("Brightness")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.alpha, 0.0..=1.0)
                    .text("Opacity")).changed();
            });

        egui::CollapsingHeader::new("🖌 Rendering")
            .default_open(true)
            .show(ui, |ui| {
                egui::ComboBox::from_label("Draw Mode")
                    .selected_text(self.draw_mode.name())
                    .show_ui(ui, |ui| {
                        for mode in DrawMode::all() {
                            if ui.selectable_value(&mut self.draw_mode, mode, mode.name()).clicked() {
                                changed = true;
                            }
                        }
                    });

                changed |= ui.add(egui::Slider::new(&mut self.line_width, 0.5..=5.0)
                    .text("Line Width")).changed();

                changed |= ui.checkbox(&mut self.show_construction, "Show Construction").changed();
            });

        egui::CollapsingHeader::new("🎬 Animation")
            .default_open(true)
            .show(ui, |ui| {
                if ui.checkbox(&mut self.animate, "Enable Animation").changed() {
                    changed = true;
                }

                if self.animate {
                    ui.add(egui::Slider::new(&mut self.animation_speed, 0.1..=5.0)
                        .text("Speed"));
                }
            });

        egui::CollapsingHeader::new("📍 Position")
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.center_x, 0.0..=1.0)
                    .text("Center X")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.center_y, 0.0..=1.0)
                    .text("Center Y")).changed();

                if ui.button("Reset Position").clicked() {
                    self.center_x = 0.5;
                    self.center_y = 0.5;
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🎯 Presets")
            .show(ui, |ui| {
                if ui.button("Classic Snowflake").clicked() {
                    *self = Self::default();
                    changed = true;
                }
                if ui.button("Rainbow Spiral").clicked() {
                    self.color_scheme = ColorScheme::Rainbow;
                    self.animate = true;
                    self.iterations = 5;
                    changed = true;
                }
                if ui.button("Fire Crystal").clicked() {
                    self.color_scheme = ColorScheme::Fire;
                    self.iterations = 6;
                    self.scale = 0.35;
                    changed = true;
                }
            });

        // Handle animation
        if self.animate {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt;
            changed = true;
        }

        changed
    }
}

impl KochSnowflake {
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
