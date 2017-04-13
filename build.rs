
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    if profile == "release" {
        let man_input_file = Path::new(&root_dir).join("man").join("panser.1.md");
        let man_output_folder = Path::new(&root_dir).join("target").join("release").join("man");
        fs::create_dir(&man_output_folder).unwrap();
        let man_output_file = man_output_folder.join("panser.1");
        let _ = Command::new("pandoc")
            .args(&["-s", "-t", "man", "-o", man_output_file.to_str().unwrap(), man_input_file.to_str().unwrap()])
            .status();
    }
}

