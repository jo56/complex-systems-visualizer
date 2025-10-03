use crate::{Color, Simulation2D};
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum Material {
    Empty,
    Sand,
    Water,
    Stone,
    Fire,
    Wood,
}

pub struct FallingSand {
    pub brush_size: usize,
    pub gravity: f32,
    pub current_material: usize,
    grid: Vec<Material>,
    width: usize,
    height: usize,
    velocities: Vec<(f32, f32)>,
    temperatures: Vec<f32>,
}

impl Default for FallingSand {
    fn default() -> Self {
        let width = 200;
        let height = 150;
        let size = width * height;

        Self {
            brush_size: 3,
            gravity: 0.5,
            current_material: 0,
            grid: vec![Material::Empty; size],
            width,
            height,
            velocities: vec![(0.0, 0.0); size],
            temperatures: vec![0.0; size],
        }
    }
}

impl FallingSand {
    pub fn new() -> Self {
        Self::default()
    }

    fn material_color(&self, material: Material, temp: f32) -> Color {
        match material {
            Material::Empty => Color::BLACK,
            Material::Sand => Color::from_rgb(194, 178, 128),
            Material::Water => Color::from_rgb(50, 100, 200),
            Material::Stone => Color::from_rgb(100, 100, 100),
            Material::Fire => {
                let intensity = (temp / 100.0).min(1.0);
                Color::from_rgb(
                    255,
                    (200.0 * (1.0 - intensity)) as u8,
                    0
                )
            }
            Material::Wood => Color::from_rgb(139, 90, 43),
        }
    }

    fn material_name(index: usize) -> &'static str {
        match index {
            0 => "Sand",
            1 => "Water",
            2 => "Stone",
            3 => "Fire",
            4 => "Wood",
            _ => "Sand",
        }
    }

    fn index_to_material(index: usize) -> Material {
        match index {
            0 => Material::Sand,
            1 => Material::Water,
            2 => Material::Stone,
            3 => Material::Fire,
            4 => Material::Wood,
            _ => Material::Sand,
        }
    }

    fn update_particle(&mut self, x: usize, y: usize) {
        let idx = y * self.width + x;
        let material = self.grid[idx];

        match material {
            Material::Empty | Material::Stone => return,
            Material::Sand => self.update_sand(x, y),
            Material::Water => self.update_water(x, y),
            Material::Fire => self.update_fire(x, y),
            Material::Wood => self.update_wood(x, y),
        }
    }

    fn update_sand(&mut self, x: usize, y: usize) {
        if y >= self.height - 1 {
            return;
        }

        let idx = y * self.width + x;
        let below = (y + 1) * self.width + x;

        // Fall straight down
        if self.grid[below] == Material::Empty || self.grid[below] == Material::Water {
            self.grid.swap(idx, below);
            self.velocities.swap(idx, below);
            self.temperatures.swap(idx, below);
        }
        // Fall diagonally
        else {
            let mut rng = rand::thread_rng();
            let dx = if rng.gen_bool(0.5) { -1 } else { 1 };
            let new_x = (x as i32 + dx) as usize;

            if new_x < self.width {
                let diag = (y + 1) * self.width + new_x;
                if self.grid[diag] == Material::Empty || self.grid[diag] == Material::Water {
                    self.grid.swap(idx, diag);
                    self.velocities.swap(idx, diag);
                    self.temperatures.swap(idx, diag);
                }
            }
        }
    }

    fn update_water(&mut self, x: usize, y: usize) {
        if y >= self.height - 1 {
            return;
        }

        let idx = y * self.width + x;
        let below = (y + 1) * self.width + x;

        // Fall down
        if self.grid[below] == Material::Empty {
            self.grid.swap(idx, below);
            self.velocities.swap(idx, below);
            self.temperatures.swap(idx, below);
        }
        // Spread sideways
        else {
            let mut rng = rand::thread_rng();
            let dx = if rng.gen_bool(0.5) { -1 } else { 1 };
            let new_x = (x as i32 + dx) as usize;

            if new_x < self.width {
                let side = y * self.width + new_x;
                if self.grid[side] == Material::Empty {
                    self.grid.swap(idx, side);
                    self.velocities.swap(idx, side);
                    self.temperatures.swap(idx, side);
                }
            }
        }
    }

    fn update_fire(&mut self, x: usize, y: usize) {
        let idx = y * self.width + x;
        let mut rng = rand::thread_rng();

        // Fire spreads and rises
        self.temperatures[idx] -= 1.0;

        if self.temperatures[idx] <= 0.0 {
            self.grid[idx] = Material::Empty;
            return;
        }

        // Rise up
        if y > 0 {
            let above = (y - 1) * self.width + x;
            if self.grid[above] == Material::Empty && rng.gen_bool(0.3) {
                self.grid.swap(idx, above);
                self.temperatures.swap(idx, above);
            }
        }

        // Ignite adjacent wood
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for (nx, ny) in neighbors {
            if nx < self.width && ny < self.height {
                let nidx = ny * self.width + nx;
                if self.grid[nidx] == Material::Wood && rng.gen_bool(0.1) {
                    self.grid[nidx] = Material::Fire;
                    self.temperatures[nidx] = 100.0;
                }
            }
        }
    }

    fn update_wood(&mut self, _x: usize, _y: usize) {
        // Wood is static unless ignited by fire
    }
}

impl Simulation2D for FallingSand {
    fn name(&self) -> &str {
        "Falling Sand"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut pixels = vec![Color::BLACK; width * height];

        for y in 0..height {
            for x in 0..width {
                let src_x = (x * self.width) / width;
                let src_y = (y * self.height) / height;
                let src_idx = src_y * self.width + src_x;

                if src_idx < self.grid.len() {
                    let material = self.grid[src_idx];
                    let temp = self.temperatures[src_idx];
                    pixels[y * width + x] = self.material_color(material, temp);
                }
            }
        }

        pixels
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        ui.heading("Falling Sand");

        egui::CollapsingHeader::new("🖌 Brush Settings")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.brush_size, 1..=10)
                    .text("Brush Size"));

                ui.label("Material:");
                ui.horizontal(|ui| {
                    for i in 0..5 {
                        if ui.selectable_label(self.current_material == i, Self::material_name(i)).clicked() {
                            self.current_material = i;
                        }
                    }
                });
            });

        egui::CollapsingHeader::new("⚙ Physics")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.gravity, 0.1..=2.0)
                    .text("Gravity"));
            });

        if ui.button("🗑 Clear").clicked() {
            self.grid.fill(Material::Empty);
            self.temperatures.fill(0.0);
        }

        // Spawn materials from top to create continuous falling effect
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) {
            let x = rng.gen_range(0..self.width);
            let y = 0;

            // Place material
            for dy in 0..self.brush_size {
                for dx in 0..self.brush_size {
                    let px = x.saturating_add(dx).saturating_sub(self.brush_size / 2);
                    let py = y + dy;

                    if px < self.width && py < self.height {
                        let idx = py * self.width + px;
                        self.grid[idx] = Self::index_to_material(self.current_material);
                        if self.current_material == 3 { // Fire
                            self.temperatures[idx] = 100.0;
                        }
                    }
                }
            }
        }

        // Update physics
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                self.update_particle(x, y);
            }
        }

        true
    }
}
