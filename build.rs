fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/vivid.ico");

        res.compile()
            .expect("Failed to compile Windows resources");
    }
}
