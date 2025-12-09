fn main() {
    // Only compile Windows resources when targeting Windows (not WASM)
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        // Check if we're cross-compiling to WASM
        let target = std::env::var("TARGET").unwrap_or_default();
        if !target.contains("wasm") {
            let mut res = winres::WindowsResource::new();
            // Only embed icon if AppIcon.ico exists (generated in CI from AppIcon.png)
            if std::path::Path::new("assets/AppIcon.ico").exists() {
                res.set_icon("assets/AppIcon.ico");
            }
            if let Err(e) = res.compile() {
                println!("cargo:warning=Failed to compile Windows resources: {}", e);
            }
        }
    }
}
