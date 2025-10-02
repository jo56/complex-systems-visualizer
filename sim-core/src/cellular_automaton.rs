use crate::{Color, Simulation2D};

pub struct CellularAutomaton {
    rule: u8,
    pub grid_width: usize,
    pub rows: usize,
    cells: Vec<Vec<bool>>,
    current_row: usize,
    pub speed: f32,
    time_accumulator: f32,
}

impl Default for CellularAutomaton {
    fn default() -> Self {
        let width = 200;
        let rows = 150;
        let mut cells = vec![vec![false; width]; rows];

        // Start with a single cell in the middle
        cells[0][width / 2] = true;

        Self {
            rule: 30,
            grid_width: width,
            rows,
            cells,
            current_row: 0,
            speed: 10.0,
            time_accumulator: 0.0,
        }
    }
}

impl CellularAutomaton {
    pub fn new(rule: u8) -> Self {
        let mut ca = Self::default();
        ca.rule = rule;
        ca
    }

    fn apply_rule(&self, left: bool, center: bool, right: bool) -> bool {
        let index = ((left as u8) << 2) | ((center as u8) << 1) | (right as u8);
        (self.rule >> index) & 1 == 1
    }

    pub fn step(&mut self) {
        if self.current_row + 1 >= self.rows {
            return;
        }

        let prev_row = self.current_row;
        let next_row = self.current_row + 1;

        for x in 0..self.grid_width {
            let left = self.cells[prev_row][(x + self.grid_width - 1) % self.grid_width];
            let center = self.cells[prev_row][x];
            let right = self.cells[prev_row][(x + 1) % self.grid_width];

            self.cells[next_row][x] = self.apply_rule(left, center, right);
        }

        self.current_row += 1;
    }

    pub fn reset(&mut self) {
        for row in &mut self.cells {
            row.fill(false);
        }
        self.cells[0][self.grid_width / 2] = true;
        self.current_row = 0;
    }

    pub fn randomize_start(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let random_state = RandomState::new();
        for i in 0..self.grid_width {
            let mut hasher = random_state.build_hasher();
            i.hash(&mut hasher);
            self.cells[0][i] = hasher.finish() % 2 == 0;
        }
        self.current_row = 0;

        // Clear rest of grid
        for row in 1..self.rows {
            self.cells[row].fill(false);
        }
    }
}

impl Simulation2D for CellularAutomaton {
    fn name(&self) -> &str {
        "Elementary Cellular Automaton"
    }

    fn compute(&self, width: usize, height: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(width * height);

        let cell_width = width / self.grid_width;
        let cell_height = height / self.rows;

        for py in 0..height {
            for px in 0..width {
                let gx = px / cell_width.max(1);
                let gy = py / cell_height.max(1);

                if gx < self.grid_width && gy < self.rows {
                    let cell = self.cells[gy][gx];
                    colors.push(if cell {
                        // Color based on rule
                        match self.rule {
                            30 => Color { r: 255, g: 150, b: 0 },
                            110 => Color { r: 100, g: 150, b: 255 },
                            90 => Color { r: 255, g: 100, b: 150 },
                            _ => Color::WHITE,
                        }
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

        ui.heading("Elementary Cellular Automaton");

        ui.horizontal(|ui| {
            ui.label("Rule:");
            if ui.add(egui::DragValue::new(&mut self.rule).range(0..=255)).changed() {
                self.reset();
                changed = true;
            }
            ui.label(format!("(binary: {:08b})", self.rule));
        });

        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                self.step();
                changed = true;
            }

            if ui.button("Reset").clicked() {
                self.reset();
                changed = true;
            }

            if ui.button("Random Start").clicked() {
                self.randomize_start();
                changed = true;
            }
        });

        ui.add(egui::Slider::new(&mut self.speed, 1.0..=60.0)
            .text("Steps per second"));

        ui.separator();
        ui.label("Famous rules:");
        if ui.button("Rule 30 (Chaotic)").clicked() {
            self.rule = 30;
            self.reset();
            changed = true;
        }
        if ui.button("Rule 110 (Turing Complete)").clicked() {
            self.rule = 110;
            self.reset();
            changed = true;
        }
        if ui.button("Rule 90 (Sierpinski Triangle)").clicked() {
            self.rule = 90;
            self.reset();
            changed = true;
        }
        if ui.button("Rule 184 (Traffic Flow)").clicked() {
            self.rule = 184;
            self.reset();
            changed = true;
        }

        // Auto-stepping
        let dt = ui.input(|i| i.stable_dt);
        self.time_accumulator += dt * self.speed;

        while self.time_accumulator >= 1.0 && self.current_row + 1 < self.rows {
            self.step();
            self.time_accumulator -= 1.0;
            changed = true;
        }

        changed
    }
}
