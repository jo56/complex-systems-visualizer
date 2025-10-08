use crate::{Color, ColorScheme, Simulation2D};
use num_complex::Complex64;
use rayon::prelude::*;

pub struct BurningShip {
    pub max_iterations: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub zoom: f64,
    pub escape_radius: f64,
    pub color_scheme: ColorScheme,
    pub smooth_coloring: bool,
    pub invert_colors: bool,
    pub color_offset: f32,
}

impl Default for BurningShip {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            center_x: -0.5,
            center_y: -0.6,
            zoom: 0.7,
            escape_radius: 2.0,
            color_scheme: ColorScheme::Fire,
            smooth_coloring: true,
            invert_colors: false,
            color_offset: 0.0,
        }
    }
}

impl BurningShip {
    pub fn new() -> Self {
        Self::default()
    }

    fn burning_ship_iterations(&self, c: Complex64) -> (u32, f64) {
        let mut z = Complex64::new(0.0, 0.0);
        let escape_sqr = self.escape_radius * self.escape_radius;

        for i in 0..self.max_iterations {
            let z_norm_sqr = z.norm_sqr();
            if z_norm_sqr > escape_sqr {
                if self.smooth_coloring {
                    let log_zn = z_norm_sqr.ln() / 2.0;
                    let nu = (log_zn / self.escape_radius.ln()).ln() / 2_f64.ln();
                    return (i, i as f64 + 1.0 - nu);
                }
                return (i, i as f64);
            }

            // Burning Ship: z = (|Re(z)| + i|Im(z)|)^2 + c
            z = Complex64::new(z.re.abs(), z.im.abs());
            z = z * z + c;
        }
        (self.max_iterations, self.max_iterations as f64)
    }

    fn pixel_to_complex(&self, x: usize, y: usize, width: usize, height: usize) -> Complex64 {
        let aspect = width as f64 / height as f64;
        let range = 4.0 / self.zoom;

        let real = self.center_x + (x as f64 / width as f64 - 0.5) * range * aspect;
        let imag = self.center_y + (y as f64 / height as f64 - 0.5) * range;

        Complex64::new(real, imag)
    }

    fn iterations_to_color(&self, iterations: u32, smooth_iter: f64) -> Color {
        if iterations == self.max_iterations {
            return Color::BLACK;
        }

        let t = ((smooth_iter / self.max_iterations as f64) as f32 + self.color_offset) % 1.0;
        let color = self.color_scheme.map(t, self.smooth_coloring);

        if self.invert_colors {
            Color::from_rgb(255 - color.r, 255 - color.g, 255 - color.b)
        } else {
            color
        }
    }
}

impl Simulation2D for BurningShip {
    fn name(&self) -> &str {
        "Burning Ship Fractal"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let c = self.pixel_to_complex(x, y, width, height);
                        let (iterations, smooth_iter) = self.burning_ship_iterations(c);
                        self.iterations_to_color(iterations, smooth_iter)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Burning Ship Fractal");

        egui::CollapsingHeader::new("⚙ Calculation Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.max_iterations, 10..=1000)
                    .text("Max Iterations")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.escape_radius, 2.0..=10.0)
                    .text("Escape Radius")).changed();
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

                changed |= ui.checkbox(&mut self.smooth_coloring, "Smooth Coloring").changed();
                changed |= ui.checkbox(&mut self.invert_colors, "Invert Colors").changed();
                changed |= ui.add(egui::Slider::new(&mut self.color_offset, 0.0..=1.0)
                    .text("Color Offset")).changed();
            });

        egui::CollapsingHeader::new("🔍 Navigation")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.zoom, 0.1..=10000.0)
                    .logarithmic(true)
                    .text("Zoom")).changed();

                ui.horizontal(|ui| {
                    ui.label("Center X:");
                    changed |= ui.add(egui::DragValue::new(&mut self.center_x)
                        .speed(0.01 / self.zoom)).changed();
                });

                ui.horizontal(|ui| {
                    ui.label("Center Y:");
                    changed |= ui.add(egui::DragValue::new(&mut self.center_y)
                        .speed(0.01 / self.zoom)).changed();
                });

                if ui.button("🏠 Reset View").clicked() {
                    *self = Self::default();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("📍 Interesting Locations")
            .show(ui, |ui| {
                if ui.button("Main Ship").clicked() {
                    self.center_x = -0.5;
                    self.center_y = -0.6;
                    self.zoom = 0.7;
                    changed = true;
                }
                if ui.button("Antenna Detail").clicked() {
                    self.center_x = -1.75;
                    self.center_y = -0.03;
                    self.zoom = 100.0;
                    changed = true;
                }
                if ui.button("Mast Detail").clicked() {
                    self.center_x = -1.762;
                    self.center_y = 0.028;
                    self.zoom = 500.0;
                    changed = true;
                }
            });

        changed
    }

    fn supports_zoom(&self) -> bool {
        true
    }

    fn adjust_center(&mut self, dx: f64, dy: f64, width: usize, height: usize) {
        // Convert pixel delta to world space delta
        let aspect = width as f64 / height as f64;
        let view_width = 4.0 / self.zoom;
        let view_height = view_width / aspect;

        self.center_x -= dx * view_width / width as f64;
        self.center_y -= dy * view_height / height as f64;
    }

    fn adjust_zoom(&mut self, delta: f64) {
        let zoom_factor = 1.0 + delta * 0.001;
        self.zoom = (self.zoom * zoom_factor).clamp(0.1, 10000.0);
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }
}
