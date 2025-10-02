use crate::{Color, Simulation2D};

pub struct GameOfLife {
    pub grid_width: usize,
    pub grid_height: usize,
    cells: Vec<bool>,
    speed: f32,
    time_accumulator: f32,
}

impl Default for GameOfLife {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let mut cells = vec![false; width * height];

        // Initialize with a glider gun pattern
        Self::add_glider_gun(&mut cells, width, 10, 10);

        Self {
            grid_width: width,
            grid_height: height,
            cells,
            speed: 10.0,
            time_accumulator: 0.0,
        }
    }
}

impl GameOfLife {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_glider_gun(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![
            (1, 5), (1, 6), (2, 5), (2, 6),
            (11, 5), (11, 6), (11, 7),
            (12, 4), (12, 8),
            (13, 3), (13, 9),
            (14, 3), (14, 9),
            (15, 6),
            (16, 4), (16, 8),
            (17, 5), (17, 6), (17, 7),
            (18, 6),
            (21, 3), (21, 4), (21, 5),
            (22, 3), (22, 4), (22, 5),
            (23, 2), (23, 6),
            (25, 1), (25, 2), (25, 6), (25, 7),
            (35, 3), (35, 4), (36, 3), (36, 4),
        ];

        for (dx, dy) in pattern {
            let idx = (y + dy) * width + (x + dx);
            if idx < cells.len() {
                cells[idx] = true;
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as i32 + dx + self.grid_width as i32) % self.grid_width as i32;
                let ny = (y as i32 + dy + self.grid_height as i32) % self.grid_height as i32;

                let idx = ny as usize * self.grid_width + nx as usize;
                if self.cells[idx] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn step(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let idx = y * self.grid_width + x;
                let neighbors = self.count_neighbors(x, y);

                new_cells[idx] = match (self.cells[idx], neighbors) {
                    (true, 2..=3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        self.cells = new_cells;
    }

    pub fn randomize(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let random_state = RandomState::new();
        for i in 0..self.cells.len() {
            let mut hasher = random_state.build_hasher();
            i.hash(&mut hasher);
            self.cells[i] = hasher.finish() % 3 == 0;
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }
}

impl Simulation2D for GameOfLife {
    fn name(&self) -> &str {
        "Conway's Game of Life"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(width * height);

        let cell_width = width / self.grid_width;
        let cell_height = height / self.grid_height;

        for py in 0..height {
            for px in 0..width {
                let gx = px / cell_width.max(1);
                let gy = py / cell_height.max(1);

                if gx < self.grid_width && gy < self.grid_height {
                    let idx = gy * self.grid_width + gx;
                    colors.push(if self.cells[idx] {
                        Color { r: 0, g: 255, b: 100 }
                    } else {
                        Color::BLACK
                    });
                } else {
                    colors.push(Color::BLACK);
                }
            }
        }

        colors
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Game of Life Controls");

        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                self.step();
                changed = true;
            }

            if ui.button("Randomize").clicked() {
                self.randomize();
                changed = true;
            }

            if ui.button("Clear").clicked() {
                self.clear();
                changed = true;
            }
        });

        ui.add(egui::Slider::new(&mut self.speed, 1.0..=60.0)
            .text("Steps per second"));

        if ui.button("Reset to Glider Gun").clicked() {
            *self = Self::default();
            changed = true;
        }

        // Auto-stepping
        let dt = ui.input(|i| i.stable_dt);
        self.time_accumulator += dt * self.speed;

        while self.time_accumulator >= 1.0 {
            self.step();
            self.time_accumulator -= 1.0;
            changed = true;
        }

        changed
    }
}
