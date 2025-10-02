pub mod mandelbrot;
pub mod julia;
pub mod game_of_life;
pub mod lorenz;
pub mod cellular_automaton;
pub mod burning_ship;
pub mod rossler;

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
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

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

    pub fn lerp(a: Color, b: Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
            g: (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
            b: (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        }
    }
}

/// Color schemes for fractal visualization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Classic,
    Rainbow,
    Fire,
    Ice,
    Grayscale,
    Ultra,
    Sunset,
    Ocean,
}

impl ColorScheme {
    pub fn all() -> Vec<ColorScheme> {
        vec![
            ColorScheme::Classic,
            ColorScheme::Rainbow,
            ColorScheme::Fire,
            ColorScheme::Ice,
            ColorScheme::Grayscale,
            ColorScheme::Ultra,
            ColorScheme::Sunset,
            ColorScheme::Ocean,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            ColorScheme::Classic => "Classic",
            ColorScheme::Rainbow => "Rainbow",
            ColorScheme::Fire => "Fire",
            ColorScheme::Ice => "Ice",
            ColorScheme::Grayscale => "Grayscale",
            ColorScheme::Ultra => "Ultra",
            ColorScheme::Sunset => "Sunset",
            ColorScheme::Ocean => "Ocean",
        }
    }

    pub fn map(&self, t: f32, smooth: bool) -> Color {
        let t = if smooth { t } else { (t * 100.0).floor() / 100.0 };
        let t = t.clamp(0.0, 1.0);

        match self {
            ColorScheme::Classic => {
                let hue = t * 360.0;
                let saturation = 1.0;
                let value = if t < 0.5 { 1.0 } else { 2.0 - 2.0 * t };
                Color::from_hsv(hue, saturation, value)
            }
            ColorScheme::Rainbow => {
                Color::from_hsv(t * 360.0, 0.8, 0.95)
            }
            ColorScheme::Fire => {
                if t < 0.33 {
                    Color::lerp(Color::BLACK, Color::RED, t * 3.0)
                } else if t < 0.66 {
                    Color::lerp(Color::RED, Color::from_rgb(255, 165, 0), (t - 0.33) * 3.0)
                } else {
                    Color::lerp(Color::from_rgb(255, 165, 0), Color::from_rgb(255, 255, 100), (t - 0.66) * 3.0)
                }
            }
            ColorScheme::Ice => {
                if t < 0.5 {
                    Color::lerp(Color::BLACK, Color::BLUE, t * 2.0)
                } else {
                    Color::lerp(Color::BLUE, Color::from_rgb(100, 200, 255), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::Grayscale => {
                let intensity = (t * 255.0) as u8;
                Color::from_rgb(intensity, intensity, intensity)
            }
            ColorScheme::Ultra => {
                let hue = (t * 720.0) % 360.0;
                Color::from_hsv(hue, 1.0, 1.0)
            }
            ColorScheme::Sunset => {
                if t < 0.25 {
                    Color::lerp(Color::from_rgb(25, 25, 112), Color::from_rgb(138, 43, 226), t * 4.0)
                } else if t < 0.5 {
                    Color::lerp(Color::from_rgb(138, 43, 226), Color::from_rgb(255, 69, 0), (t - 0.25) * 4.0)
                } else if t < 0.75 {
                    Color::lerp(Color::from_rgb(255, 69, 0), Color::from_rgb(255, 215, 0), (t - 0.5) * 4.0)
                } else {
                    Color::lerp(Color::from_rgb(255, 215, 0), Color::from_rgb(255, 255, 200), (t - 0.75) * 4.0)
                }
            }
            ColorScheme::Ocean => {
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(0, 20, 40), Color::from_rgb(0, 105, 148), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(0, 105, 148), Color::from_rgb(72, 209, 204), (t - 0.5) * 2.0)
                }
            }
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
