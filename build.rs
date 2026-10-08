fn main() {
    slint_build::compile_with_config(
        "ui/app-window.slint",
        slint_build::CompilerConfiguration::new()
            .with_include_paths(vec![std::path::PathBuf::from("ui/components")]),
    ).expect("Slint build failed");
}
