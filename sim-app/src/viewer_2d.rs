use eframe::egui;
use sim_core::Simulation2D;

pub struct Viewer2D {
    pub needs_update: bool,
    texture: Option<egui::TextureHandle>,
    width: usize,
    height: usize,
}

impl Viewer2D {
    pub fn new() -> Self {
        Self {
            needs_update: true,
            texture: None,
            width: 800,
            height: 600,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, simulation: &Box<dyn Simulation2D>) {
        let available_size = ui.available_size();
        let width = available_size.x as usize;
        let height = available_size.y as usize;

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

        // Display texture
        if let Some(texture) = &self.texture {
            let size = egui::vec2(width as f32, height as f32);
            ui.image((texture.id(), size));
        }
    }
}
