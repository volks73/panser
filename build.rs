use std::env;
use std::path::PathBuf;
use std::process::Command;

const MAN_NAME: &str = "man";
const MAN_EXT: &str = "1";
const MARKDOWN_EXT: &str = "md";
const PANDOC: &str = "pandoc";
const HTML_EXT: &str = "html";

fn main() {
    let profile = env::var("PROFILE").unwrap();
    if profile == "release" {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
        let mut output_man_file = PathBuf::from(&root_dir);
        output_man_file.push(MAN_NAME);
        output_man_file.push(&pkg_name);
        output_man_file.set_extension(MAN_EXT);
        let mut output_html_file = PathBuf::from(&root_dir);
        output_html_file.push(MAN_NAME);
        output_html_file.push("manpage");
        output_html_file.set_extension(HTML_EXT);
        let mut input_doc_file = PathBuf::from(&root_dir);
        input_doc_file.push(MAN_NAME);
        input_doc_file.push(format!("{}.{}.{}", &pkg_name, MAN_EXT, MARKDOWN_EXT));
        // If there is an error building the command, such as the `pandoc` executable is not found,
        // i.e. not installed, then just ignore the error. In other words, if building fails
        // because pandoc is not installed then do not worry about building the
        // documentation/manpage. However, if the process of building the documentation fails,
        // panic to let the power user know the documentation did not get built. This is because
        // the `cargo install` command uses the release profile but a user may not have pandoc
        // installed as it is optional. Eventually, it looks like Cargo will have manpage support
        // for binaries using the `cargo install` command, but it is still under discussion and
        // development. This is a workaround until that is implemented. Note, this does not
        // actually install the manpage it just builds it when the `cargo build --release` command
        // is used.
        if let Some(status) = Command::new(PANDOC)
            .arg("-s")
            .arg("-t")
            .arg(MAN_NAME)
            .arg("-o")
            .arg(&output_man_file)
            .arg(&input_doc_file)
            .status()
            .ok() {
            if !status.success() {
                panic!("Failed to build the groff manpage for release");
            }
        }
        if let Some(status) = Command::new(PANDOC)
            .arg("-s")
            .arg("-t")
            .arg("html")
            .arg("-o")
            .arg(&output_html_file)
            .arg(&input_doc_file)
            .status()
            .ok() {
            if !status.success() {
                panic!("Failed to build the html manpage for release");
            }
        }
    }
}

