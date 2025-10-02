#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    // Only embed icon if AppIcon.ico exists (generated in CI from AppIcon.png)
    if std::path::Path::new("assets/AppIcon.ico").exists() {
        res.set_icon("assets/AppIcon.ico");
    }
    res.compile().expect("Failed to compile Windows resources");
}

#[cfg(not(windows))]
fn main() {
    // No-op on non-Windows builds
}
