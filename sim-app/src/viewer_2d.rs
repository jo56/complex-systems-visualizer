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
    is_dragging: bool,
    last_mouse_pos: Option<egui::Pos2>,
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
            is_dragging: false,
            last_mouse_pos: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, simulation: &Box<dyn Simulation2D>) {
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
            let image_size = egui::vec2(width as f32, height as f32);
            let display_size = egui::vec2(available_size.x, available_size.y);

            // Create a scrollable area if image is larger than display
            let response = ui.allocate_rect(
                egui::Rect::from_min_size(ui.cursor().min, display_size),
                egui::Sense::click_and_drag(),
            );

            // Handle dragging for panning
            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    if let Some(last_pos) = self.last_mouse_pos {
                        let delta = pointer_pos - last_pos;
                        self.pan_x += delta.x;
                        self.pan_y += delta.y;

                        // Clamp panning to keep image somewhat visible
                        let max_pan_x = (image_size.x - display_size.x).max(0.0);
                        let max_pan_y = (image_size.y - display_size.y).max(0.0);
                        self.pan_x = self.pan_x.clamp(-max_pan_x, 0.0);
                        self.pan_y = self.pan_y.clamp(-max_pan_y, 0.0);
                    }
                    self.last_mouse_pos = Some(pointer_pos);
                    self.is_dragging = true;
                }
            } else {
                self.last_mouse_pos = None;
                self.is_dragging = false;
            }

            // Calculate UV coordinates for the visible portion
            let uv_min = if image_size.x > display_size.x || image_size.y > display_size.y {
                egui::pos2(
                    (-self.pan_x / image_size.x).max(0.0),
                    (-self.pan_y / image_size.y).max(0.0),
                )
            } else {
                egui::pos2(0.0, 0.0)
            };

            let uv_max = if image_size.x > display_size.x || image_size.y > display_size.y {
                egui::pos2(
                    ((-self.pan_x + display_size.x) / image_size.x).min(1.0),
                    ((-self.pan_y + display_size.y) / image_size.y).min(1.0),
                )
            } else {
                egui::pos2(1.0, 1.0)
            };

            // Draw the image with UV mapping for panning
            let rect = response.rect;
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(uv_min, uv_max),
                egui::Color32::WHITE,
            );
        }
    }
}
