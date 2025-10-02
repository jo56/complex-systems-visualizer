use crate::{Color, Simulation2D};
use num_complex::Complex64;
use rayon::prelude::*;

pub struct Julia {
    pub max_iterations: u32,
    pub c_real: f64,
    pub c_imag: f64,
    pub zoom: f64,
    pub colorize: bool,
}

impl Default for Julia {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            c_real: -0.7,
            c_imag: 0.27015,
            zoom: 1.0,
            colorize: true,
        }
    }
}

impl Julia {
    pub fn new() -> Self {
        Self::default()
    }

    fn julia_iterations(&self, z0: Complex64) -> u32 {
        let c = Complex64::new(self.c_real, self.c_imag);
        let mut z = z0;

        for i in 0..self.max_iterations {
            if z.norm_sqr() > 4.0 {
                return i;
            }
            z = z * z + c;
        }
        self.max_iterations
    }

    fn pixel_to_complex(&self, x: usize, y: usize, width: usize, height: usize) -> Complex64 {
        let aspect = width as f64 / height as f64;
        let range = 4.0 / self.zoom;

        let real = (x as f64 / width as f64 - 0.5) * range * aspect;
        let imag = (y as f64 / height as f64 - 0.5) * range;

        Complex64::new(real, imag)
    }

    fn iterations_to_color(&self, iterations: u32) -> Color {
        if iterations == self.max_iterations {
            Color::BLACK
        } else if self.colorize {
            let t = iterations as f32 / self.max_iterations as f32;
            let hue = (t * 360.0 + 200.0) % 360.0;
            let saturation = 0.8;
            let value = if t < 0.5 { 0.5 + t } else { 1.5 - t };
            Color::from_hsv(hue, saturation, value)
        } else {
            let intensity = (255.0 * (1.0 - iterations as f32 / self.max_iterations as f32)) as u8;
            Color { r: intensity, g: intensity, b: intensity }
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
                        let iterations = self.julia_iterations(z);
                        self.iterations_to_color(iterations)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Julia Set Parameters");

        changed |= ui.add(egui::Slider::new(&mut self.max_iterations, 10..=500)
            .text("Max Iterations")).changed();

        changed |= ui.add(egui::Slider::new(&mut self.zoom, 0.1..=100.0)
            .logarithmic(true)
            .text("Zoom")).changed();

        ui.horizontal(|ui| {
            ui.label("C Real:");
            changed |= ui.add(egui::DragValue::new(&mut self.c_real)
                .speed(0.001)
                .range(-2.0..=2.0)).changed();
        });

        ui.horizontal(|ui| {
            ui.label("C Imaginary:");
            changed |= ui.add(egui::DragValue::new(&mut self.c_imag)
                .speed(0.001)
                .range(-2.0..=2.0)).changed();
        });

        changed |= ui.checkbox(&mut self.colorize, "Colorize").changed();

        if ui.button("Reset").clicked() {
            *self = Self::default();
            changed = true;
        }

        ui.separator();
        ui.label("Interesting parameters:");
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

        changed
    }
}
