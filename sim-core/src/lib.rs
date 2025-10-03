pub mod mandelbrot;
pub mod julia;
pub mod game_of_life;
pub mod lorenz;
pub mod cellular_automaton;
pub mod burning_ship;
pub mod rossler;

// Additional 3D attractors
pub mod aizawa;
pub mod halvorsen;
pub mod dadras;
pub mod thomas;
pub mod chen;

// Animated 2D simulations
pub mod double_pendulum;
pub mod reaction_diffusion;
pub mod lissajous;
pub mod wave_interference;

// Generative patterns module
pub mod generative;

// New complexity simulations
pub mod langtons_ant;
pub mod cyclic_ca;
pub mod dla;
pub mod sandpile;

// Enhanced 3D simulations
pub mod particle_attractor_3d;
pub mod boids_3d;

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

    pub fn rgb_to_hsv_static(color: Color) -> (f32, f32, f32) {
        let r = color.r as f32 / 255.0;
        let g = color.g as f32 / 255.0;
        let b = color.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
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
    // New palettes
    Plasma,
    Viridis,
    Inferno,
    Magma,
    Cividis,
    Turbo,
    CoolWarm,
    Spectral,
    Purple,
    Green,
    Blues,
    YellowOrangeBrown,
    PinkYellow,
    Neon,
    Pastel,
    Earth,
    Copper,
    Galaxy,
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
            ColorScheme::Plasma,
            ColorScheme::Viridis,
            ColorScheme::Inferno,
            ColorScheme::Magma,
            ColorScheme::Cividis,
            ColorScheme::Turbo,
            ColorScheme::CoolWarm,
            ColorScheme::Spectral,
            ColorScheme::Purple,
            ColorScheme::Green,
            ColorScheme::Blues,
            ColorScheme::YellowOrangeBrown,
            ColorScheme::PinkYellow,
            ColorScheme::Neon,
            ColorScheme::Pastel,
            ColorScheme::Earth,
            ColorScheme::Copper,
            ColorScheme::Galaxy,
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
            ColorScheme::Plasma => "Plasma",
            ColorScheme::Viridis => "Viridis",
            ColorScheme::Inferno => "Inferno",
            ColorScheme::Magma => "Magma",
            ColorScheme::Cividis => "Cividis",
            ColorScheme::Turbo => "Turbo",
            ColorScheme::CoolWarm => "Cool-Warm",
            ColorScheme::Spectral => "Spectral",
            ColorScheme::Purple => "Purple",
            ColorScheme::Green => "Green",
            ColorScheme::Blues => "Blues",
            ColorScheme::YellowOrangeBrown => "Yellow-Orange-Brown",
            ColorScheme::PinkYellow => "Pink-Yellow",
            ColorScheme::Neon => "Neon",
            ColorScheme::Pastel => "Pastel",
            ColorScheme::Earth => "Earth",
            ColorScheme::Copper => "Copper",
            ColorScheme::Galaxy => "Galaxy",
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
            ColorScheme::Plasma => {
                // Plasma: Purple → Pink → Orange → Yellow
                if t < 0.33 {
                    Color::lerp(Color::from_rgb(13, 8, 135), Color::from_rgb(183, 55, 121), t * 3.0)
                } else if t < 0.66 {
                    Color::lerp(Color::from_rgb(183, 55, 121), Color::from_rgb(252, 136, 68), (t - 0.33) * 3.0)
                } else {
                    Color::lerp(Color::from_rgb(252, 136, 68), Color::from_rgb(240, 249, 33), (t - 0.66) * 3.0)
                }
            }
            ColorScheme::Viridis => {
                // Viridis: Dark Blue → Teal → Yellow-Green
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(68, 1, 84), Color::from_rgb(59, 82, 139), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(59, 82, 139), Color::from_rgb(253, 231, 37), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::Inferno => {
                // Inferno: Black → Purple → Red → Yellow
                if t < 0.33 {
                    Color::lerp(Color::from_rgb(0, 0, 4), Color::from_rgb(106, 23, 110), t * 3.0)
                } else if t < 0.66 {
                    Color::lerp(Color::from_rgb(106, 23, 110), Color::from_rgb(237, 93, 36), (t - 0.33) * 3.0)
                } else {
                    Color::lerp(Color::from_rgb(237, 93, 36), Color::from_rgb(252, 255, 164), (t - 0.66) * 3.0)
                }
            }
            ColorScheme::Magma => {
                // Magma: Black → Purple → Pink → White
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(0, 0, 4), Color::from_rgb(124, 48, 147), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(124, 48, 147), Color::from_rgb(252, 253, 191), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::Cividis => {
                // Cividis: Blue → Yellow (colorblind-friendly)
                Color::lerp(Color::from_rgb(0, 32, 77), Color::from_rgb(253, 231, 97), t)
            }
            ColorScheme::Turbo => {
                // Turbo: Rainbow-like but more perceptually uniform
                Color::from_hsv(t * 300.0, 0.9, 0.95)
            }
            ColorScheme::CoolWarm => {
                // Cool-Warm: Blue → White → Red
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(59, 76, 192), Color::from_rgb(221, 221, 221), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(221, 221, 221), Color::from_rgb(180, 4, 38), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::Spectral => {
                // Spectral: Red → Yellow → Green → Blue → Purple
                Color::from_hsv((1.0 - t) * 280.0, 0.8, 0.9)
            }
            ColorScheme::Purple => {
                // Purple gradient
                Color::lerp(Color::from_rgb(30, 0, 50), Color::from_rgb(200, 100, 255), t)
            }
            ColorScheme::Green => {
                // Green gradient
                Color::lerp(Color::from_rgb(0, 50, 20), Color::from_rgb(100, 255, 150), t)
            }
            ColorScheme::Blues => {
                // Blues gradient
                Color::lerp(Color::from_rgb(8, 29, 88), Color::from_rgb(158, 202, 225), t)
            }
            ColorScheme::YellowOrangeBrown => {
                // Yellow → Orange → Brown
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(255, 255, 178), Color::from_rgb(254, 178, 76), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(254, 178, 76), Color::from_rgb(127, 59, 8), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::PinkYellow => {
                // Pink → Yellow
                Color::lerp(Color::from_rgb(255, 105, 180), Color::from_rgb(255, 255, 100), t)
            }
            ColorScheme::Neon => {
                // Bright neon colors
                Color::from_hsv(t * 360.0, 1.0, 1.0)
            }
            ColorScheme::Pastel => {
                // Soft pastel colors
                Color::from_hsv(t * 360.0, 0.3, 0.95)
            }
            ColorScheme::Earth => {
                // Earth tones: Brown → Tan → Green
                if t < 0.5 {
                    Color::lerp(Color::from_rgb(101, 67, 33), Color::from_rgb(194, 178, 128), t * 2.0)
                } else {
                    Color::lerp(Color::from_rgb(194, 178, 128), Color::from_rgb(135, 169, 107), (t - 0.5) * 2.0)
                }
            }
            ColorScheme::Copper => {
                // Copper: Black → Brown → Orange
                Color::lerp(Color::BLACK, Color::from_rgb(255, 138, 76), t)
            }
            ColorScheme::Galaxy => {
                // Galaxy: Deep space colors
                if t < 0.33 {
                    Color::lerp(Color::from_rgb(10, 5, 30), Color::from_rgb(70, 30, 100), t * 3.0)
                } else if t < 0.66 {
                    Color::lerp(Color::from_rgb(70, 30, 100), Color::from_rgb(130, 80, 200), (t - 0.33) * 3.0)
                } else {
                    Color::lerp(Color::from_rgb(130, 80, 200), Color::from_rgb(200, 150, 255), (t - 0.66) * 3.0)
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
