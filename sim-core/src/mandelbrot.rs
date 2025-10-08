use crate::{Color, ColorScheme, Simulation2D};
use num_complex::Complex64;
use rayon::prelude::*;

pub struct Mandelbrot {
    pub max_iterations: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub zoom: f64,
    pub power: f64,
    pub escape_radius: f64,
    pub color_scheme: ColorScheme,
    pub smooth_coloring: bool,
    pub invert_colors: bool,
    pub color_offset: f32,
    pub color_cycling: bool,
    cycle_time: f32,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
            power: 2.0,
            escape_radius: 2.0,
            color_scheme: ColorScheme::Classic,
            smooth_coloring: true,
            invert_colors: false,
            color_offset: 0.0,
            color_cycling: false,
            cycle_time: 0.0,
        }
    }
}

impl Mandelbrot {
    pub fn new() -> Self {
        Self::default()
    }

    fn mandelbrot_iterations(&self, c: Complex64) -> (u32, f64) {
        let mut z = Complex64::new(0.0, 0.0);
        let escape_sqr = self.escape_radius * self.escape_radius;

        for i in 0..self.max_iterations {
            let z_norm_sqr = z.norm_sqr();
            if z_norm_sqr > escape_sqr {
                if self.smooth_coloring {
                    // Smooth iteration count using continuous coloring
                    let log_zn = z_norm_sqr.ln() / 2.0;
                    let nu = (log_zn / self.escape_radius.ln()).ln() / 2_f64.ln();
                    return (i, i as f64 + 1.0 - nu);
                }
                return (i, i as f64);
            }

            // z = z^power + c (generalized Mandelbrot)
            if (self.power - 2.0).abs() < 0.001 {
                z = z * z + c;
            } else {
                z = z.powf(self.power) + c;
            }
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

        let mut t = (smooth_iter / self.max_iterations as f64) as f32;

        // Apply color offset/cycling
        if self.color_cycling {
            t = (t + self.cycle_time) % 1.0;
        } else {
            t = (t + self.color_offset) % 1.0;
        }

        let color = self.color_scheme.map(t, self.smooth_coloring);

        if self.invert_colors {
            Color::from_rgb(255 - color.r, 255 - color.g, 255 - color.b)
        } else {
            color
        }
    }
}

impl Simulation2D for Mandelbrot {
    fn name(&self) -> &str {
        "Mandelbrot Set"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let c = self.pixel_to_complex(x, y, width, height);
                        let (iterations, smooth_iter) = self.mandelbrot_iterations(c);
                        self.iterations_to_color(iterations, smooth_iter)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Mandelbrot Set");

        egui::CollapsingHeader::new("⚙ Calculation Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.add(egui::Slider::new(&mut self.max_iterations, 10..=1000)
                    .text("Max Iterations")).changed();

                changed |= ui.add(egui::Slider::new(&mut self.power, 2.0..=8.0)
                    .text("Power (z^n)")).changed();

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

                if ui.add(egui::Slider::new(&mut self.color_offset, 0.0..=1.0)
                    .text("Color Offset")).changed() {
                    changed = true;
                }

                if ui.checkbox(&mut self.color_cycling, "Animate Colors").changed() {
                    changed = true;
                }
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
                if ui.button("Seahorse Valley").clicked() {
                    self.center_x = -0.75;
                    self.center_y = 0.1;
                    self.zoom = 100.0;
                    changed = true;
                }
                if ui.button("Elephant Valley").clicked() {
                    self.center_x = 0.3;
                    self.center_y = 0.0;
                    self.zoom = 50.0;
                    changed = true;
                }
                if ui.button("Spiral").clicked() {
                    self.center_x = -0.7269;
                    self.center_y = 0.1889;
                    self.zoom = 500.0;
                    changed = true;
                }
                if ui.button("Triple Spiral").clicked() {
                    self.center_x = -0.1011;
                    self.center_y = 0.9563;
                    self.zoom = 1000.0;
                    changed = true;
                }
                if ui.button("Mini Mandelbrot").clicked() {
                    self.center_x = -0.7453;
                    self.center_y = 0.1127;
                    self.zoom = 5000.0;
                    changed = true;
                }
            });

        // Handle color cycling animation
        if self.color_cycling {
            let dt = ui.input(|i| i.stable_dt);
            self.cycle_time = (self.cycle_time + dt * 0.1) % 1.0;
            changed = true;
        }

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
        // Adjust zoom with exponential scaling for smooth mousewheel control
        let zoom_factor = 1.0 + delta * 0.001;
        self.zoom = (self.zoom * zoom_factor).clamp(0.1, 10000.0);
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }
}
