#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Assicurati che res/rnetbench.ico esista
    let mut res = WindowsResource::new();
    res.set_icon("assets/rnetbench.ico")
        .set("FileDescription", "rNetBench CLI")
        .set("ProductName", "rNetBench")
        .set("OriginalFilename", "rnetbench.exe")
        .set("FileVersion", env!("CARGO_PKG_VERSION"))
        .set("ProductVersion", env!("CARGO_PKG_VERSION"))
        .compile()
        .expect("Failed to embed icon resource");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
