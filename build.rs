use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or(String::from("")) != "wasm32" {
        return;
    }

    let osciwasm_path = Path::new("./tools/osciwasm");
    Command::new("npm")
        .args(&["i"])
        .current_dir(&osciwasm_path)
        .status()
        .expect("npm install failed");
    Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&osciwasm_path)
        .status()
        .expect("Building osciwasm JavaScript bindings failed");
}
