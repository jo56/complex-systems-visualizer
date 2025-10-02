use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use sim_core::Simulation3D;

pub struct Viewer3D {
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,
}

impl Viewer3D {
    pub fn new() -> Self {
        Self {
            rotation_x: 0.3,
            rotation_y: 0.7,
            zoom: 1.0,
        }
    }

    fn rotate_point(&self, point: [f32; 3]) -> [f32; 3] {
        let [x, y, z] = point;

        // Rotate around Y axis
        let cos_y = self.rotation_y.cos();
        let sin_y = self.rotation_y.sin();
        let x1 = x * cos_y - z * sin_y;
        let z1 = x * sin_y + z * cos_y;

        // Rotate around X axis
        let cos_x = self.rotation_x.cos();
        let sin_x = self.rotation_x.sin();
        let y1 = y * cos_x - z1 * sin_x;
        let z2 = y * sin_x + z1 * cos_x;

        [x1, y1, z2]
    }

    fn project_3d_to_2d(&self, point: [f32; 3]) -> [f64; 2] {
        let rotated = self.rotate_point(point);
        let [x, y, _z] = rotated;
        [x as f64 * self.zoom as f64, y as f64 * self.zoom as f64]
    }

    pub fn show(&mut self, ui: &mut egui::Ui, simulation: &Box<dyn Simulation3D>) {
        ui.vertical(|ui| {
            // Controls
            ui.horizontal(|ui| {
                ui.label("Rotation X:");
                ui.add(egui::Slider::new(&mut self.rotation_x, 0.0..=std::f32::consts::TAU));
                ui.label("Rotation Y:");
                ui.add(egui::Slider::new(&mut self.rotation_y, 0.0..=std::f32::consts::TAU));
            });

            ui.horizontal(|ui| {
                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0));
                if ui.button("Reset View").clicked() {
                    self.rotation_x = 0.3;
                    self.rotation_y = 0.7;
                    self.zoom = 1.0;
                }
            });

            ui.separator();

            // Get 3D points and project to 2D
            let points_3d = simulation.get_points();

            if points_3d.is_empty() {
                ui.label("Generating visualization...");
                return;
            }

            // Project all points
            let points_2d: Vec<[f64; 2]> = points_3d
                .iter()
                .map(|&p| self.project_3d_to_2d(p))
                .collect();

            let plot_points = PlotPoints::new(points_2d);

            // Create plot
            Plot::new("3d_plot")
                .show_axes(true)
                .data_aspect(1.0)
                .allow_drag(true)
                .allow_zoom(true)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    // Draw trail with gradient color
                    let line = Line::new(plot_points)
                        .color(egui::Color32::from_rgb(100, 200, 255))
                        .width(1.5);
                    plot_ui.line(line);

                    // Draw current point
                    if let Some(&last) = points_3d.last() {
                        let projected = self.project_3d_to_2d(last);
                        let point = PlotPoints::new(vec![projected]);
                        let points_plot = egui_plot::Points::new(point)
                            .color(egui::Color32::from_rgb(255, 100, 100))
                            .radius(5.0);
                        plot_ui.points(points_plot);
                    }
                });
        });
    }
}
