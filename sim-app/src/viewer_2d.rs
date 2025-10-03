use eframe::egui;
use sim_core::Simulation2D;

pub struct Viewer2D {
    pub needs_update: bool,
    pub scale: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    texture: Option<egui::TextureHandle>,
    width: usize,
    height: usize,
}

impl Viewer2D {
    pub fn new() -> Self {
        Self {
            needs_update: true,
            scale: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            texture: None,
            width: 800,
            height: 600,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, simulation: &mut Box<dyn Simulation2D>) {
        let available_size = ui.available_size();
        let width = (available_size.x * self.scale) as usize;
        let height = (available_size.y * self.scale) as usize;

        // Check if we need to recompute
        if self.needs_update || width != self.width || height != self.height {
            self.width = width;
            self.height = height;

            // Compute simulation
            let colors = simulation.compute(width, height);

            // Convert to egui color image
            let pixels: Vec<egui::Color32> = colors
                .iter()
                .map(|c| egui::Color32::from_rgb(c.r, c.g, c.b))
                .collect();

            let color_image = egui::ColorImage {
                size: [width, height],
                pixels,
            };

            // Update or create texture
            if let Some(texture) = &mut self.texture {
                texture.set(color_image, Default::default());
            } else {
                self.texture = Some(ui.ctx().load_texture(
                    "simulation",
                    color_image,
                    Default::default(),
                ));
            }

            self.needs_update = false;
        }

        // Create an interactive area for the image
        if let Some(texture) = &self.texture {
            let display_size = egui::vec2(available_size.x, available_size.y);

            // Create a scrollable area if image is larger than display
            let response = ui.allocate_rect(
                egui::Rect::from_min_size(ui.cursor().min, display_size),
                egui::Sense::click_and_drag(),
            );

            // Handle dragging for panning
            if response.dragged() && simulation.supports_zoom() {
                let delta = response.drag_delta();
                // Adjust the simulation's center position
                simulation.adjust_center(delta.x as f64, delta.y as f64, width, height);
                self.needs_update = true;
            }

            // Draw the image
            let rect = response.rect;
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}
