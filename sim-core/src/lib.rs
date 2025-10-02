pub mod mandelbrot;
pub mod julia;
pub mod game_of_life;
pub mod lorenz;
pub mod cellular_automaton;

/// Color representation in RGB format
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }
}

/// Trait for 2D simulations that produce colored images
pub trait Simulation2D: Send + Sync {
    /// Get the name of the simulation
    fn name(&self) -> &str;

    /// Compute the simulation and return pixel data
    fn compute(&self, width: usize, height: usize) -> Vec<Color>;

    /// Get UI parameters for egui controls
    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool;
}

/// Trait for 3D simulations
pub trait Simulation3D: Send + Sync {
    /// Get the name of the simulation
    fn name(&self) -> &str;

    /// Step the simulation forward
    fn step(&mut self, dt: f32);

    /// Get current 3D points for visualization
    fn get_points(&self) -> Vec<[f32; 3]>;

    /// Get UI parameters for egui controls
    fn ui_parameters(&mut self, ui: &mut egui::Ui) -> bool;

    /// Reset simulation to initial state
    fn reset(&mut self);
}

pub type Simulation2DBox = Box<dyn Simulation2D>;
pub type Simulation3DBox = Box<dyn Simulation3D>;
