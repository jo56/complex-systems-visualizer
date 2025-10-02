use crate::{Color, Simulation2D};

#[derive(Clone, Copy, PartialEq)]
pub enum LifeRule {
    Conway,      // B3/S23
    HighLife,    // B36/S23
    Seeds,       // B2/S
    LifeWithoutDeath, // B3/S012345678
    DayAndNight, // B3678/S34678
    Maze,        // B3/S12345
}

impl LifeRule {
    pub fn all() -> Vec<LifeRule> {
        vec![
            LifeRule::Conway,
            LifeRule::HighLife,
            LifeRule::Seeds,
            LifeRule::LifeWithoutDeath,
            LifeRule::DayAndNight,
            LifeRule::Maze,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            LifeRule::Conway => "Conway (B3/S23)",
            LifeRule::HighLife => "HighLife (B36/S23)",
            LifeRule::Seeds => "Seeds (B2/S)",
            LifeRule::LifeWithoutDeath => "Life Without Death (B3/S012345678)",
            LifeRule::DayAndNight => "Day & Night (B3678/S34678)",
            LifeRule::Maze => "Maze (B3/S12345)",
        }
    }

    pub fn should_live(&self, alive: bool, neighbors: u8) -> bool {
        match self {
            LifeRule::Conway => match (alive, neighbors) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            },
            LifeRule::HighLife => match (alive, neighbors) {
                (true, 2..=3) => true,
                (false, 3 | 6) => true,
                _ => false,
            },
            LifeRule::Seeds => !alive && neighbors == 2,
            LifeRule::LifeWithoutDeath => alive || neighbors == 3,
            LifeRule::DayAndNight => match (alive, neighbors) {
                (true, 3..=4 | 6..=8) => true,
                (false, 3 | 6..=8) => true,
                _ => false,
            },
            LifeRule::Maze => match (alive, neighbors) {
                (true, 1..=5) => true,
                (false, 3) => true,
                _ => false,
            },
        }
    }
}

pub struct GameOfLife {
    pub grid_width: usize,
    pub grid_height: usize,
    cells: Vec<bool>,
    cell_age: Vec<u32>,
    speed: f32,
    time_accumulator: f32,
    pub rule: LifeRule,
    pub show_age: bool,
    pub paused: bool,
    generation: u64,
}

impl Default for GameOfLife {
    fn default() -> Self {
        let width = 120;
        let height = 120;
        let mut cells = vec![false; width * height];
        let cell_age = vec![0; width * height];

        // Initialize with a glider gun pattern
        Self::add_glider_gun(&mut cells, width, 10, 10);

        Self {
            grid_width: width,
            grid_height: height,
            cells,
            cell_age,
            speed: 10.0,
            time_accumulator: 0.0,
            rule: LifeRule::Conway,
            show_age: false,
            paused: false,
            generation: 0,
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
        let mut new_age = self.cell_age.clone();

        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let idx = y * self.grid_width + x;
                let neighbors = self.count_neighbors(x, y);

                new_cells[idx] = self.rule.should_live(self.cells[idx], neighbors);

                // Update age
                if new_cells[idx] {
                    new_age[idx] = if self.cells[idx] {
                        self.cell_age[idx].saturating_add(1)
                    } else {
                        1
                    };
                } else {
                    new_age[idx] = 0;
                }
            }
        }

        self.cells = new_cells;
        self.cell_age = new_age;
        self.generation += 1;
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.clear();
        let cx = self.grid_width / 2;
        let cy = self.grid_height / 2;

        match pattern {
            "Glider Gun" => Self::add_glider_gun(&mut self.cells, self.grid_width, 10, 10),
            "Glider" => Self::add_glider(&mut self.cells, self.grid_width, cx, cy),
            "Pulsar" => Self::add_pulsar(&mut self.cells, self.grid_width, cx, cy),
            "Pentadecathlon" => Self::add_pentadecathlon(&mut self.cells, self.grid_width, cx, cy),
            "Lwss" => Self::add_lwss(&mut self.cells, self.grid_width, cx, cy),
            "Acorn" => Self::add_acorn(&mut self.cells, self.grid_width, cx, cy),
            _ => {}
        }
        self.generation = 0;
    }

    fn add_glider(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in pattern {
            if x + dx < width && y + dy < width {
                cells[(y + dy) * width + (x + dx)] = true;
            }
        }
    }

    fn add_pulsar(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![
            // Top
            (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
            // Upper ring
            (0,2),(5,2),(7,2),(12,2),
            (0,3),(5,3),(7,3),(12,3),
            (0,4),(5,4),(7,4),(12,4),
            // Middle
            (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
            (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
            // Lower ring
            (0,8),(5,8),(7,8),(12,8),
            (0,9),(5,9),(7,9),(12,9),
            (0,10),(5,10),(7,10),(12,10),
            // Bottom
            (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
        ];
        for (dx, dy) in pattern {
            if x + dx < width && y + dy < width {
                cells[(y + dy) * width + (x + dx)] = true;
            }
        }
    }

    fn add_pentadecathlon(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![
            (0,1),(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),
            (0,0),(2,0),(5,0),(7,0),
            (0,2),(2,2),(5,2),(7,2),
        ];
        for (dx, dy) in pattern {
            if x + dx < width && y + dy < width {
                cells[(y + dy) * width + (x + dx)] = true;
            }
        }
    }

    fn add_lwss(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![
            (1,0),(4,0),(0,1),(0,2),(4,2),(0,3),(1,3),(2,3),(3,3),
        ];
        for (dx, dy) in pattern {
            if x + dx < width && y + dy < width {
                cells[(y + dy) * width + (x + dx)] = true;
            }
        }
    }

    fn add_acorn(cells: &mut Vec<bool>, width: usize, x: usize, y: usize) {
        let pattern = vec![(1,0),(3,1),(0,2),(1,2),(4,2),(5,2),(6,2)];
        for (dx, dy) in pattern {
            if x + dx < width && y + dy < width {
                cells[(y + dy) * width + (x + dx)] = true;
            }
        }
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
                        if self.show_age {
                            // Color by age
                            let age = self.cell_age[idx].min(50) as f32 / 50.0;
                            Color::from_hsv(age * 240.0, 0.8, 0.9)
                        } else {
                            Color { r: 0, g: 255, b: 100 }
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

        ui.heading("Conway's Game of Life");

        egui::CollapsingHeader::new("🎮 Controls")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(if self.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
                        self.paused = !self.paused;
                    }

                    if ui.button("⏭ Step").clicked() {
                        self.step();
                        changed = true;
                    }

                    if ui.button("🔄 Clear").clicked() {
                        self.clear();
                        self.generation = 0;
                        changed = true;
                    }
                });

                ui.add(egui::Slider::new(&mut self.speed, 1.0..=60.0)
                    .text("Steps/Second"));

                ui.label(format!("Generation: {}", self.generation));
                ui.label(format!("Live Cells: {}", self.cells.iter().filter(|&&c| c).count()));
            });

        egui::CollapsingHeader::new("📐 Rules")
            .default_open(true)
            .show(ui, |ui| {
                egui::ComboBox::from_label("Rule Set")
                    .selected_text(self.rule.name())
                    .show_ui(ui, |ui| {
                        for rule in LifeRule::all() {
                            if ui.selectable_value(&mut self.rule, rule, rule.name()).clicked() {
                                changed = true;
                            }
                        }
                    });
            });

        egui::CollapsingHeader::new("🎨 Visualization")
            .default_open(true)
            .show(ui, |ui| {
                if ui.checkbox(&mut self.show_age, "Color by Age").changed() {
                    changed = true;
                }
            });

        egui::CollapsingHeader::new("🧬 Pattern Library")
            .show(ui, |ui| {
                if ui.button("Glider Gun").clicked() {
                    self.add_pattern("Glider Gun");
                    changed = true;
                }
                if ui.button("Glider").clicked() {
                    self.add_pattern("Glider");
                    changed = true;
                }
                if ui.button("Pulsar").clicked() {
                    self.add_pattern("Pulsar");
                    changed = true;
                }
                if ui.button("Pentadecathlon").clicked() {
                    self.add_pattern("Pentadecathlon");
                    changed = true;
                }
                if ui.button("LWSS (Spaceship)").clicked() {
                    self.add_pattern("Lwss");
                    changed = true;
                }
                if ui.button("Acorn").clicked() {
                    self.add_pattern("Acorn");
                    changed = true;
                }
                if ui.button("Random Soup").clicked() {
                    self.randomize();
                    self.generation = 0;
                    changed = true;
                }
            });

        // Auto-stepping
        if !self.paused {
            let dt = ui.input(|i| i.stable_dt);
            self.time_accumulator += dt * self.speed;

            while self.time_accumulator >= 1.0 {
                self.step();
                self.time_accumulator -= 1.0;
                changed = true;
            }
        }

        changed
    }
}
