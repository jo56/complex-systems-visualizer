use crate::{Color, ColorScheme, Simulation2D};
use num_complex::Complex64;
use rayon::prelude::*;

pub struct Julia {
    pub max_iterations: u32,
    pub c_real: f64,
    pub c_imag: f64,
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub power: f64,
    pub escape_radius: f64,
    pub color_scheme: ColorScheme,
    pub smooth_coloring: bool,
    pub invert_colors: bool,
    pub color_offset: f32,
    pub animate: bool,
    animation_time: f32,
    animation_radius: f64,
}

impl Default for Julia {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            c_real: -0.7,
            c_imag: 0.27015,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
            power: 2.0,
            escape_radius: 2.0,
            color_scheme: ColorScheme::Ultra,
            smooth_coloring: true,
            invert_colors: false,
            color_offset: 0.0,
            animate: false,
            animation_time: 0.0,
            animation_radius: 0.7885,
        }
    }
}

impl Julia {
    pub fn new() -> Self {
        Self::default()
    }

    fn julia_iterations(&self, z0: Complex64) -> (u32, f64) {
        let c = Complex64::new(self.c_real, self.c_imag);
        let mut z = z0;
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

        let real = (x as f64 / width as f64 - 0.5) * range * aspect + self.center_x;
        let imag = (y as f64 / height as f64 - 0.5) * range + self.center_y;

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

impl Simulation2D for Julia {
    fn name(&self) -> &str {
        "Julia Set"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let z = self.pixel_to_complex(x, y, width, height);
                        let (iterations, smooth_iter) = self.julia_iterations(z);
                        self.iterations_to_color(iterations, smooth_iter)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Julia Set");

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

        egui::CollapsingHeader::new("🌀 Julia Parameter (c)")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Real:");
                    changed |= ui.add(egui::DragValue::new(&mut self.c_real)
                        .speed(0.001)
                        .range(-2.0..=2.0)).changed();
                });

                ui.horizontal(|ui| {
                    ui.label("Imaginary:");
                    changed |= ui.add(egui::DragValue::new(&mut self.c_imag)
                        .speed(0.001)
                        .range(-2.0..=2.0)).changed();
                });

                if ui.checkbox(&mut self.animate, "Animate c parameter").changed() {
                    changed = true;
                }

                if self.animate {
                    ui.add(egui::Slider::new(&mut self.animation_radius, 0.1..=1.0)
                        .text("Animation Radius"));
                }
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
                if ui.button("🏠 Reset").clicked() {
                    *self = Self::default();
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("📍 Interesting Parameters")
            .show(ui, |ui| {
                if ui.button("Dendrite").clicked() {
                    self.c_real = -0.4;
                    self.c_imag = 0.6;
                    changed = true;
                }
                if ui.button("San Marco Dragon").clicked() {
                    self.c_real = -0.75;
                    self.c_imag = 0.0;
                    changed = true;
                }
                if ui.button("Siegel Disk").clicked() {
                    self.c_real = -0.391;
                    self.c_imag = -0.587;
                    changed = true;
                }
                if ui.button("Douady's Rabbit").clicked() {
                    self.c_real = -0.123;
                    self.c_imag = 0.745;
                    changed = true;
                }
                if ui.button("Galaxy").clicked() {
                    self.c_real = 0.285;
                    self.c_imag = 0.01;
                    changed = true;
                }
            });

        // Handle animation
        if self.animate {
            let dt = ui.input(|i| i.stable_dt);
            self.animation_time += dt * 0.3;
            self.c_real = self.animation_radius * self.animation_time.cos() as f64;
            self.c_imag = self.animation_radius * self.animation_time.sin() as f64;
            changed = true;
        }

        changed
    }
}
