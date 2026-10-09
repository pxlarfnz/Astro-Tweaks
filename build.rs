
use std::path::Path;

fn main() {
    #[cfg(windows)]
    {
        let icon_path = "assets/astro.ico";

        println!("cargo:rerun-if-changed={icon_path}");

        if !Path::new(icon_path).is_file() {
            panic!(
                "Windows icon not found: {}. \
                 Make sure assets/astro.ico exists in the project root.",
                icon_path
            );
        }

        let mut resource = winres::WindowsResource::new();

        resource.set_icon(icon_path);

        resource
            .compile()
            .expect("Failed to compile Windows resources");
    }
}
