//! WebAssembly entry point for the Complex Systems Visualizer

use log::LevelFilter;
use wasm_bindgen::{prelude::*, JsCast};

/// Entry point for WASM - called from JavaScript
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    // Initialize better diagnostics for the browser console
    console_error_panic_hook::set_once();
    eframe::WebLogger::init(LevelFilter::Info).ok();

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| JsValue::from_str("No canvas element with id 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Element is not a canvas"))?;

    // Loading overlay is hidden once the app is created
    let loading = document
        .get_element_by_id("loading")
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());

    let web_options = eframe::WebOptions::default();

    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(move |cc| {
                if let Some(loading) = loading {
                    let _ = loading.style().set_property("display", "none");
                }
                Ok(Box::new(sim_app::ComplexSystemsApp::new(cc)))
            }),
        )
        .await
        .map_err(|err| JsValue::from_str(&err.to_string()))
}
