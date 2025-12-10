//! sim-app library - Complex Systems Visualizer
//!
//! This library exports the main application components for use by both
//! the native desktop app and the WebAssembly web app.

pub mod viewer_2d;
pub mod viewer_3d;

use eframe::egui;
use sim_core::*;

/// Simulation type selector
#[derive(Clone, Copy, PartialEq)]
pub enum SimulationType {
    TwoD,
    ThreeD,
}

/// Main application state
pub struct ComplexSystemsApp {
    pub sim_type: SimulationType,
    pub sim_2d_index: usize,
    pub sim_3d_index: usize,
    pub simulations_2d: Vec<Simulation2DBox>,
    pub simulations_3d: Vec<Simulation3DBox>,
    pub viewer_2d: viewer_2d::Viewer2D,
    pub viewer_3d: viewer_3d::Viewer3D,
}

impl ComplexSystemsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let simulations_2d: Vec<Simulation2DBox> = vec![
            // Fractals
            Box::new(mandelbrot::Mandelbrot::new()),
            Box::new(julia::Julia::new()),
            Box::new(burning_ship::BurningShip::new()),

            // Cellular Systems & Emergent Complexity
            Box::new(game_of_life::GameOfLife::new()),
            Box::new(cellular_automaton::CellularAutomaton::new(30)),
            Box::new(langtons_ant::LangtonsAnt::new()),
            Box::new(cyclic_ca::CyclicCA::new()),

            // Growth & Self-Organization
            Box::new(dla::DLA::new()),
            Box::new(sandpile::Sandpile::new()),

            // Animated Simulations
            Box::new(double_pendulum::DoublePendulum::new()),
            Box::new(reaction_diffusion::ReactionDiffusion::new()),
            Box::new(lissajous::LissajousCurves::new()),
            Box::new(wave_interference::WaveInterference::new()),

            // Generative Patterns
            Box::new(generative::KochSnowflake::new()),
            Box::new(generative::Phyllotaxis::new()),
            Box::new(generative::PerlinFlow::new()),
            Box::new(generative::Boids::new()),
            Box::new(generative::DeJongAttractor::new()),
            Box::new(generative::CliffordAttractor::new()),

            // Complex Emergent Simulations
            Box::new(slime_mold::SlimeMold::new()),
            Box::new(falling_sand::FallingSand::new()),
        ];

        let simulations_3d: Vec<Simulation3DBox> = vec![
            // Stunning 3D Visualizations
            Box::new(dna_helix::DNAHelix::new()),
            Box::new(torus_knot::TorusKnot::new()),
            Box::new(galaxy_spiral::GalaxySpiral::new()),

            // Enhanced Particle Systems
            Box::new(particle_attractor_3d::ParticleAttractor3D::new()),
            Box::new(boids_3d::Boids3D::new()),

            // Classic Attractors
            Box::new(lorenz::LorenzAttractor::new()),
            Box::new(rossler::RosslerAttractor::new()),

            // Additional Chaotic Attractors
            Box::new(aizawa::AizawaAttractor::new()),
            Box::new(halvorsen::HalvorsenAttractor::new()),
            Box::new(dadras::DadrasAttractor::new()),
            Box::new(thomas::ThomasAttractor::new()),
            Box::new(chen::ChenAttractor::new()),

            // Diverse Particle Simulations
            Box::new(nbody_gravity::NBodyGravity::new()),
            Box::new(fluid_sph::FluidSPH::new()),
            Box::new(magnetic_field::MagneticField::new()),

            // Radical 3D Animations
            Box::new(vortex_turbulence::VortexTurbulence::new()),
            Box::new(lightning_bolt::LightningBolt::new()),
            Box::new(fractal_tree_3d::FractalTree3D::new()),
        ];

        Self {
            sim_type: SimulationType::TwoD,
            sim_2d_index: 0,
            sim_3d_index: 0,
            simulations_2d,
            simulations_3d,
            viewer_2d: viewer_2d::Viewer2D::new(),
            viewer_3d: viewer_3d::Viewer3D::new(),
        }
    }
}

impl eframe::App for ComplexSystemsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::SidePanel::left("control_panel")
            .min_width(400.0)
            .max_width(550.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Complex Systems Visualizer");
                });
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("View Mode:");
                    if ui.selectable_label(matches!(self.sim_type, SimulationType::TwoD), "2D").clicked() {
                        self.sim_type = SimulationType::TwoD;
                    }
                    if ui.selectable_label(matches!(self.sim_type, SimulationType::ThreeD), "3D").clicked() {
                        self.sim_type = SimulationType::ThreeD;
                    }
                });

                ui.separator();

                // Global scale/zoom controls
                match self.sim_type {
                    SimulationType::TwoD => {
                        ui.horizontal(|ui| {
                            ui.label("Pattern Detail:");
                            if ui.add(egui::Slider::new(&mut self.viewer_2d.scale, 0.25..=2.0)
                                .text("Scale")).changed() {
                                // Clamp value to ensure it stays within valid range
                                self.viewer_2d.scale = self.viewer_2d.scale.clamp(0.25, 2.0);
                                self.viewer_2d.needs_update = true;
                                // Reset pan when scale changes to prevent shift
                                self.viewer_2d.pan_x = 0.0;
                                self.viewer_2d.pan_y = 0.0;
                            }
                            if ui.button("Reset Scale").clicked() {
                                self.viewer_2d.scale = 1.0;
                                self.viewer_2d.needs_update = true;
                                self.viewer_2d.pan_x = 0.0;
                                self.viewer_2d.pan_y = 0.0;
                            }
                            if ui.button("Reset Pan").clicked() {
                                self.viewer_2d.pan_x = 0.0;
                                self.viewer_2d.pan_y = 0.0;
                            }
                        });
                        ui.label(format!("Resolution: {}x{} pixels",
                            (800.0 * self.viewer_2d.scale) as i32,
                            (600.0 * self.viewer_2d.scale) as i32));
                        ui.label("Tip: Drag to pan (fractals), mousewheel to zoom (fractals)");
                    }
                    SimulationType::ThreeD => {
                        ui.horizontal(|ui| {
                            ui.label("View Zoom:");
                            if ui.add(egui::Slider::new(&mut self.viewer_3d.zoom, 0.5..=5.0)
                                .text("Zoom")).changed() {
                                // Clamp value to ensure it stays within valid range
                                self.viewer_3d.zoom = self.viewer_3d.zoom.clamp(0.5, 5.0);
                            }
                        });
                        ui.label("Tip: Mousewheel to zoom");
                    }
                }

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {

                    match self.sim_type {
                        SimulationType::TwoD => {
                            egui::ComboBox::from_label("Select Simulation")
                                .selected_text(self.simulations_2d[self.sim_2d_index].name())
                                .show_ui(ui, |ui| {
                                    for (i, sim) in self.simulations_2d.iter().enumerate() {
                                        if ui.selectable_value(&mut self.sim_2d_index, i, sim.name()).clicked() {
                                            self.viewer_2d.needs_update = true;
                                        }
                                    }
                                });

                            ui.separator();

                            if self.simulations_2d[self.sim_2d_index].ui_parameters(ui) {
                                self.viewer_2d.needs_update = true;
                            }
                        }
                        SimulationType::ThreeD => {
                            egui::ComboBox::from_label("Select Simulation")
                                .selected_text(self.simulations_3d[self.sim_3d_index].name())
                                .show_ui(ui, |ui| {
                                    for (i, sim) in self.simulations_3d.iter().enumerate() {
                                        ui.selectable_value(&mut self.sim_3d_index, i, sim.name());
                                    }
                                });

                            ui.separator();

                            self.simulations_3d[self.sim_3d_index].ui_parameters(ui);
                        }
                    }
                });

                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Built with ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and Rust");
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.sim_type {
                SimulationType::TwoD => {
                    self.viewer_2d.show(ui, &mut self.simulations_2d[self.sim_2d_index]);
                }
                SimulationType::ThreeD => {
                    let dt = ui.input(|i| i.stable_dt);
                    self.simulations_3d[self.sim_3d_index].step(dt);
                    self.viewer_3d.show(ui, &self.simulations_3d[self.sim_3d_index]);
                }
            }
        });
    }
}
