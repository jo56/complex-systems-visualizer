mod viewer_2d;
mod viewer_3d;

use eframe::egui;
use sim_core::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("Complex Systems Visualizer"),
        ..Default::default()
    };

    eframe::run_native(
        "Complex Systems Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(ComplexSystemsApp::new(cc)))),
    )
}

enum SimulationType {
    TwoD,
    ThreeD,
}

struct ComplexSystemsApp {
    sim_type: SimulationType,
    sim_2d_index: usize,
    sim_3d_index: usize,
    simulations_2d: Vec<Simulation2DBox>,
    simulations_3d: Vec<Simulation3DBox>,
    viewer_2d: viewer_2d::Viewer2D,
    viewer_3d: viewer_3d::Viewer3D,
}

impl ComplexSystemsApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let simulations_2d: Vec<Simulation2DBox> = vec![
            Box::new(mandelbrot::Mandelbrot::new()),
            Box::new(julia::Julia::new()),
            Box::new(burning_ship::BurningShip::new()),
            Box::new(game_of_life::GameOfLife::new()),
            Box::new(cellular_automaton::CellularAutomaton::new(30)),
        ];

        let simulations_3d: Vec<Simulation3DBox> = vec![
            Box::new(lorenz::LorenzAttractor::new()),
            Box::new(rossler::RosslerAttractor::new()),
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
            .min_width(320.0)
            .max_width(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🌌 Complex Systems Visualizer");
                });
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("📊 View Mode:");
                    if ui.selectable_label(matches!(self.sim_type, SimulationType::TwoD), "🖼 2D").clicked() {
                        self.sim_type = SimulationType::TwoD;
                    }
                    if ui.selectable_label(matches!(self.sim_type, SimulationType::ThreeD), "🎲 3D").clicked() {
                        self.sim_type = SimulationType::ThreeD;
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {

                    match self.sim_type {
                        SimulationType::TwoD => {
                            egui::ComboBox::from_label("🎨 Select Simulation")
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
                            egui::ComboBox::from_label("🌀 Select Simulation")
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
                    self.viewer_2d.show(ui, &self.simulations_2d[self.sim_2d_index]);
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
