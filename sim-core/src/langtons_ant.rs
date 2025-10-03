use crate::{Color, ColorScheme, Simulation2D};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub struct LangtonsAnt {
    pub grid_width: usize,
    pub grid_height: usize,
    pub speed: f32,
    pub color_by_age: bool,
    pub color_scheme: ColorScheme,
    pub trail_color: bool,
    pub show_ant: bool,
    pub wrap_edges: bool,

    grid: Vec<bool>,
    ant_x: i32,
    ant_y: i32,
    direction: Direction,
    time_accumulator: f32,
    step_count: usize,
}

impl Default for LangtonsAnt {
    fn default() -> Self {
        let width = 200;
        let height = 150;

        Self {
            grid_width: width,
            grid_height: height,
            speed: 100.0,
            color_by_age: false,
            color_scheme: ColorScheme::Fire,
            trail_color: false,
            show_ant: true,
            wrap_edges: true,
            grid: vec![false; width * height],
            ant_x: (width / 2) as i32,
            ant_y: (height / 2) as i32,
            direction: Direction::Up,
            time_accumulator: 0.0,
            step_count: 0,
        }
    }
}

impl LangtonsAnt {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        let idx = (self.ant_y as usize * self.grid_width + self.ant_x as usize)
                  % (self.grid_width * self.grid_height);

        let is_white = self.grid[idx];

        // Flip the color of the current square
        self.grid[idx] = !is_white;

        // Turn based on color: white = right, black = left
        self.direction = if is_white {
            self.direction.turn_right()
        } else {
            self.direction.turn_left()
        };

        // Move forward
        let (dx, dy) = self.direction.to_offset();
        self.ant_x += dx;
        self.ant_y += dy;

        // Handle edges
        if self.wrap_edges {
            if self.ant_x < 0 {
                self.ant_x += self.grid_width as i32;
            } else if self.ant_x >= self.grid_width as i32 {
                self.ant_x -= self.grid_width as i32;
            }

            if self.ant_y < 0 {
                self.ant_y += self.grid_height as i32;
            } else if self.ant_y >= self.grid_height as i32 {
                self.ant_y -= self.grid_height as i32;
            }
        } else {
            // Bounce off walls
            if self.ant_x < 0 || self.ant_x >= self.grid_width as i32 {
                self.ant_x = self.ant_x.clamp(0, self.grid_width as i32 - 1);
                self.direction = match self.direction {
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                    d => d,
                };
            }

            if self.ant_y < 0 || self.ant_y >= self.grid_height as i32 {
                self.ant_y = self.ant_y.clamp(0, self.grid_height as i32 - 1);
                self.direction = match self.direction {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    d => d,
                };
            }
        }

        self.step_count += 1;
    }

    pub fn reset(&mut self) {
        // Resize grid if dimensions changed
        let required_size = self.grid_width * self.grid_height;
        if self.grid.len() != required_size {
            self.grid = vec![false; required_size];
        } else {
            self.grid.fill(false);
        }
        self.ant_x = (self.grid_width / 2) as i32;
        self.ant_y = (self.grid_height / 2) as i32;
        self.direction = Direction::Up;
        self.step_count = 0;
        self.time_accumulator = 0.0;
    }
}

impl Simulation2D for LangtonsAnt {
    fn name(&self) -> &str {
        "Langton's Ant"
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
                    let is_white = self.grid[gy * self.grid_width + gx];

                    // Check if ant is on this cell
                    let is_ant = self.show_ant &&
                                 gx == self.ant_x as usize &&
                                 gy == self.ant_y as usize;

                    if is_ant {
                        // Draw ant in red
                        colors.push(Color::RED);
                    } else if is_white {
                        if self.trail_color {
                            // Color based on position for rainbow effect
                            let t = (gx as f32 / self.grid_width as f32 +
                                    gy as f32 / self.grid_height as f32) / 2.0;
                            colors.push(self.color_scheme.map(t, true));
                        } else {
                            colors.push(Color::WHITE);
                        }
                    } else {
                        colors.push(Color::BLACK);
                    }
                } else {
                    colors.push(Color::BLACK);
                }
            }
        }

        colors
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Langton's Ant");

        ui.label(format!("Steps: {}", self.step_count));

        if self.step_count < 10000 {
            ui.label("Phase: Chaotic (building highway)");
        } else {
            ui.label("Phase: Highway (repeating pattern)");
        }

        ui.separator();

        egui::CollapsingHeader::new("⚙ Simulation Settings")
            .default_open(true)
            .show(ui, |ui| {
                if ui.add(egui::Slider::new(&mut self.grid_width, 50..=400)
                    .text("Grid Width")).changed() {
                    self.reset();
                    changed = true;
                }

                if ui.add(egui::Slider::new(&mut self.grid_height, 50..=300)
                    .text("Grid Height")).changed() {
                    self.reset();
                    changed = true;
                }

                ui.add(egui::Slider::new(&mut self.speed, 1.0..=1000.0)
                    .logarithmic(true)
                    .text("Steps per second"));

                changed |= ui.checkbox(&mut self.wrap_edges, "Wrap Edges").changed();
            });

        egui::CollapsingHeader::new("🎨 Visual Settings")
            .default_open(true)
            .show(ui, |ui| {
                changed |= ui.checkbox(&mut self.show_ant, "Show Ant").changed();
                changed |= ui.checkbox(&mut self.trail_color, "Colored Trail").changed();

                if self.trail_color {
                    egui::ComboBox::from_label("Color Scheme")
                        .selected_text(self.color_scheme.name())
                        .show_ui(ui, |ui| {
                            for scheme in ColorScheme::all() {
                                if ui.selectable_value(&mut self.color_scheme, scheme, scheme.name()).clicked() {
                                    changed = true;
                                }
                            }
                        });
                }
            });

        egui::CollapsingHeader::new("📊 Info")
            .show(ui, |ui| {
                ui.label("Langton's Ant follows simple rules:");
                ui.label("• On white: turn right, flip color, move");
                ui.label("• On black: turn left, flip color, move");
                ui.label("");
                ui.label("It creates chaotic behavior for ~10,000");
                ui.label("steps, then builds a recurring 'highway'");
                ui.label("pattern - emergent complexity from");
                ui.label("simple rules!");
            });

        if ui.button("🔄 Reset").clicked() {
            self.reset();
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
