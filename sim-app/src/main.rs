// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use sim_app::ComplexSystemsApp;

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/AppIcon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load embedded icon")
        .to_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_position([200.0, 50.0])
            .with_title("Complex Systems Visualizer")
            .with_icon(std::sync::Arc::new(load_icon())),
        ..Default::default()
    };

    eframe::run_native(
        "Complex Systems Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(ComplexSystemsApp::new(cc)))),
    )
}
