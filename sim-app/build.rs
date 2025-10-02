fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/AppIcon.ico"); // generated in CI
    res.compile().unwrap();
}