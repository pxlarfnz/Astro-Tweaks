fn main() {
    #[cfg(windows)]
    {
        let mut resource = winres::WindowsResource::new();

        resource.set_icon("logo.ico");

        resource
            .compile()
            .expect("Failed to compile Windows resources");
    }
}