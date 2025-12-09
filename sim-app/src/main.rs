// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use sim_app::ComplexSystemsApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_position([200.0, 50.0])
            .with_title("Complex Systems Visualizer"),
        ..Default::default()
    };

    eframe::run_native(
        "Complex Systems Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(ComplexSystemsApp::new(cc)))),
    )
}
