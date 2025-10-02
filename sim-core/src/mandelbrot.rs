use crate::{Color, Simulation2D};
use num_complex::Complex64;
use rayon::prelude::*;

pub struct Mandelbrot {
    pub max_iterations: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub zoom: f64,
    pub colorize: bool,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
            colorize: true,
        }
    }
}

impl Mandelbrot {
    pub fn new() -> Self {
        Self::default()
    }

    fn mandelbrot_iterations(&self, c: Complex64) -> u32 {
        let mut z = Complex64::new(0.0, 0.0);
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

        let real = self.center_x + (x as f64 / width as f64 - 0.5) * range * aspect;
        let imag = self.center_y + (y as f64 / height as f64 - 0.5) * range;

        Complex64::new(real, imag)
    }

    fn iterations_to_color(&self, iterations: u32) -> Color {
        if iterations == self.max_iterations {
            Color::BLACK
        } else if self.colorize {
            let t = iterations as f32 / self.max_iterations as f32;
            let hue = t * 360.0;
            let saturation = 1.0;
            let value = if t < 0.5 { 1.0 } else { 2.0 - 2.0 * t };
            Color::from_hsv(hue, saturation, value)
        } else {
            let intensity = (255.0 * (1.0 - iterations as f32 / self.max_iterations as f32)) as u8;
            Color { r: intensity, g: intensity, b: intensity }
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
                        let iterations = self.mandelbrot_iterations(c);
                        self.iterations_to_color(iterations)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Mandelbrot Set Parameters");

        changed |= ui.add(egui::Slider::new(&mut self.max_iterations, 10..=500)
            .text("Max Iterations")).changed();

        changed |= ui.add(egui::Slider::new(&mut self.zoom, 0.1..=1000.0)
            .logarithmic(true)
            .text("Zoom")).changed();

        ui.horizontal(|ui| {
            ui.label("Center X:");
            changed |= ui.add(egui::DragValue::new(&mut self.center_x)
                .speed(0.01)).changed();
        });

        ui.horizontal(|ui| {
            ui.label("Center Y:");
            changed |= ui.add(egui::DragValue::new(&mut self.center_y)
                .speed(0.01)).changed();
        });

        changed |= ui.checkbox(&mut self.colorize, "Colorize").changed();

        if ui.button("Reset View").clicked() {
            *self = Self::default();
            changed = true;
        }

        ui.separator();
        ui.label("Interesting locations:");
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

        changed
    }
}
