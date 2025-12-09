//! WebAssembly entry point for the Complex Systems Visualizer

use wasm_bindgen::prelude::*;

/// Entry point for WASM - called from JavaScript
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    // Initialize panic hook for better error messages in browser console
    console_error_panic_hook::set_once();

    // Get the canvas element from the DOM
    let document = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("No canvas element with id 'canvas'")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Element is not a canvas");

    // Start the eframe web app
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(sim_app::ComplexSystemsApp::new(cc)))),
            )
            .await
            .expect("Failed to start eframe");
    });

    Ok(())
}
