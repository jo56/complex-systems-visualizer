use eframe::egui;
use sim_core::Simulation3D;

pub struct Viewer3D {
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,
    auto_rotate: bool,
    point_size: f32,
    color_mode: ColorMode,
    background_style: BackgroundStyle,
    texture: Option<egui::TextureHandle>,
}

#[derive(Clone, Copy, PartialEq)]
enum ColorMode {
    Rainbow,
    Depth,
    Velocity,
    Solid,
}

#[derive(Clone, Copy, PartialEq)]
enum BackgroundStyle {
    Gradient,
    Black,
    Stars,
}

impl Viewer3D {
    pub fn new() -> Self {
        Self {
            rotation_x: 0.3,
            rotation_y: 0.7,
            zoom: 1.5,
            auto_rotate: true,
            point_size: 4.0,
            color_mode: ColorMode::Rainbow,
            background_style: BackgroundStyle::Black,
            texture: None,
        }
    }

    fn rotate_point(&self, point: [f32; 3]) -> [f32; 3] {
        let [x, y, z] = point;

        // Rotate around Y axis
        let cos_y = self.rotation_y.cos();
        let sin_y = self.rotation_y.sin();
        let x1 = x * cos_y - z * sin_y;
        let z1 = x * sin_y + z * cos_y;

        // Rotate around X axis
        let cos_x = self.rotation_x.cos();
        let sin_x = self.rotation_x.sin();
        let y1 = y * cos_x - z1 * sin_x;
        let z2 = y * sin_x + z1 * cos_x;

        [x1, y1, z2]
    }

    fn project_to_screen(&self, point: [f32; 3], width: f32, height: f32) -> (f32, f32, f32) {
        let rotated = self.rotate_point(point);
        let [x, y, z] = rotated;

        // Perspective projection
        let perspective = 200.0 / (200.0 + z);

        let screen_x = width / 2.0 + x * self.zoom * perspective;
        let screen_y = height / 2.0 + y * self.zoom * perspective;

        (screen_x, screen_y, z)
    }

    fn draw_sphere(&self, pixels: &mut [egui::Color32], width: usize, height: usize,
                   cx: f32, cy: f32, radius: f32, color: egui::Color32, glow: bool) {
        let min_x = ((cx - radius).max(0.0) as usize).min(width - 1);
        let max_x = ((cx + radius).min(width as f32 - 1.0) as usize).min(width - 1);
        let min_y = ((cy - radius).max(0.0) as usize).min(height - 1);
        let max_y = ((cy + radius).min(height as f32 - 1.0) as usize).min(height - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= radius {
                    let idx = y * width + x;

                    if glow {
                        // Soft glow effect
                        let intensity = (1.0 - dist / radius).powf(2.0);
                        let current = pixels[idx];
                        pixels[idx] = egui::Color32::from_rgba_premultiplied(
                            ((current.r() as f32 + color.r() as f32 * intensity).min(255.0)) as u8,
                            ((current.g() as f32 + color.g() as f32 * intensity).min(255.0)) as u8,
                            ((current.b() as f32 + color.b() as f32 * intensity).min(255.0)) as u8,
                            255,
                        );
                    } else {
                        // Spherical shading
                        let intensity = (1.0 - (dist / radius) * 0.5).max(0.3);
                        pixels[idx] = egui::Color32::from_rgb(
                            (color.r() as f32 * intensity) as u8,
                            (color.g() as f32 * intensity) as u8,
                            (color.b() as f32 * intensity) as u8,
                        );
                    }
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, simulation: &Box<dyn Simulation3D>) {
        // Controls
        ui.horizontal(|ui| {
            ui.label("Rotation:");
            ui.add(egui::Slider::new(&mut self.rotation_x, 0.0..=std::f32::consts::TAU).text("X"));
            ui.add(egui::Slider::new(&mut self.rotation_y, 0.0..=std::f32::consts::TAU).text("Y"));
        });

        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut self.zoom, 0.5..=5.0).text("Zoom"));
            ui.add(egui::Slider::new(&mut self.point_size, 1.0..=15.0).text("Size"));
            ui.checkbox(&mut self.auto_rotate, "Auto-Rotate");
        });

        ui.horizontal(|ui| {
            ui.label("Color:");
            egui::ComboBox::from_label("")
                .selected_text(match self.color_mode {
                    ColorMode::Rainbow => "Rainbow",
                    ColorMode::Depth => "Depth",
                    ColorMode::Velocity => "Velocity",
                    ColorMode::Solid => "Solid",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.color_mode, ColorMode::Rainbow, "Rainbow");
                    ui.selectable_value(&mut self.color_mode, ColorMode::Depth, "Depth");
                    ui.selectable_value(&mut self.color_mode, ColorMode::Velocity, "Velocity");
                    ui.selectable_value(&mut self.color_mode, ColorMode::Solid, "Solid");
                });

            ui.label("Background:");
            egui::ComboBox::from_label(" ")
                .selected_text(match self.background_style {
                    BackgroundStyle::Gradient => "Gradient",
                    BackgroundStyle::Black => "Black",
                    BackgroundStyle::Stars => "Stars",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.background_style, BackgroundStyle::Gradient, "Gradient");
                    ui.selectable_value(&mut self.background_style, BackgroundStyle::Black, "Black");
                    ui.selectable_value(&mut self.background_style, BackgroundStyle::Stars, "Stars");
                });
        });

        ui.separator();

        // Auto-rotation
        if self.auto_rotate {
            let dt = ui.input(|i| i.stable_dt);
            self.rotation_y += dt * 0.3;
            if self.rotation_y > std::f32::consts::TAU {
                self.rotation_y -= std::f32::consts::TAU;
            }
        }

        // Get 3D points
        let points_3d = simulation.get_points();

        if points_3d.is_empty() {
            ui.label("Generating visualization...");
            return;
        }

        // Get available space and render
        let available_size = ui.available_size();
        let width = available_size.x as usize;
        let height = available_size.y as usize;

        if width == 0 || height == 0 {
            return;
        }

        // Create pixel buffer
        let mut pixels = vec![egui::Color32::BLACK; width * height];

        // Render background
        match self.background_style {
            BackgroundStyle::Gradient => {
                for y in 0..height {
                    let t = y as f32 / height as f32;
                    let color = egui::Color32::from_rgb(
                        (10.0 + t * 30.0) as u8,
                        (10.0 + t * 40.0) as u8,
                        (30.0 + t * 80.0) as u8,
                    );
                    for x in 0..width {
                        pixels[y * width + x] = color;
                    }
                }
            }
            BackgroundStyle::Stars => {
                use std::collections::hash_map::RandomState;
                use std::hash::{BuildHasher, Hash, Hasher};
                let random_state = RandomState::new();

                for i in 0..200 {
                    let mut hasher = random_state.build_hasher();
                    i.hash(&mut hasher);
                    let x = (hasher.finish() % width as u64) as usize;

                    let mut hasher = random_state.build_hasher();
                    (i + 1000).hash(&mut hasher);
                    let y = (hasher.finish() % height as u64) as usize;

                    let mut hasher = random_state.build_hasher();
                    (i + 2000).hash(&mut hasher);
                    let brightness = (hasher.finish() % 128 + 127) as u8;

                    if x < width && y < height {
                        pixels[y * width + x] = egui::Color32::from_rgb(brightness, brightness, brightness);
                    }
                }
            }
            BackgroundStyle::Black => {}
        }

        // Project and sort points by depth (back to front)
        let mut projected: Vec<(f32, f32, f32, usize)> = points_3d
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let (x, y, z) = self.project_to_screen(p, width as f32, height as f32);
                (x, y, z, i)
            })
            .collect();

        projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        // Draw all points as spheres with glow
        for (screen_x, screen_y, z, i) in projected.iter() {
            if *screen_x < 0.0 || *screen_x >= width as f32 || *screen_y < 0.0 || *screen_y >= height as f32 {
                continue;
            }

            let t = *i as f32 / points_3d.len() as f32;

            let color = match self.color_mode {
                ColorMode::Rainbow => {
                    let hue = t * 360.0;
                    let (r, g, b) = hsv_to_rgb(hue, 0.9, 1.0);
                    egui::Color32::from_rgb(r, g, b)
                }
                ColorMode::Depth => {
                    let z_norm = (*z + 100.0) / 200.0;
                    let intensity = z_norm.clamp(0.0, 1.0);
                    egui::Color32::from_rgb(
                        (255.0 * (1.0 - intensity)) as u8,
                        (150.0 * intensity) as u8,
                        (255.0 * intensity) as u8,
                    )
                }
                ColorMode::Velocity => {
                    if *i > 0 && *i < points_3d.len() {
                        let dx = points_3d[*i][0] - points_3d[i.saturating_sub(1)][0];
                        let dy = points_3d[*i][1] - points_3d[i.saturating_sub(1)][1];
                        let dz = points_3d[*i][2] - points_3d[i.saturating_sub(1)][2];
                        let velocity = (dx * dx + dy * dy + dz * dz).sqrt();
                        let vel_t = (velocity * 0.5).min(1.0);
                        egui::Color32::from_rgb(
                            (255.0 * vel_t) as u8,
                            (200.0 * (1.0 - vel_t)) as u8,
                            (100.0) as u8,
                        )
                    } else {
                        egui::Color32::from_rgb(100, 200, 100)
                    }
                }
                ColorMode::Solid => egui::Color32::from_rgb(100, 200, 255),
            };

            // Calculate size with perspective
            let perspective_scale = 200.0 / (200.0 + z);
            let radius = self.point_size * perspective_scale.max(0.2);

            self.draw_sphere(&mut pixels, width, height, *screen_x, *screen_y, radius, color, true);
        }

        // Create texture from pixels
        let color_image = egui::ColorImage {
            size: [width, height],
            pixels,
        };

        if let Some(texture) = &mut self.texture {
            texture.set(color_image, Default::default());
        } else {
            self.texture = Some(ui.ctx().load_texture(
                "3d_view",
                color_image,
                Default::default(),
            ));
        }

        // Display texture
        if let Some(texture) = &self.texture {
            let size = egui::vec2(width as f32, height as f32);
            ui.image((texture.id(), size));
        }
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
