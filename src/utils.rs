use std::io;
use std::fs::File;
use std::path::Path;
use memory::Memory;
use loader::{hexloader, rawloader, LoadError, Result};

pub fn load_file(filename: &Path) -> Result<Box<Memory>> {
    let mut file = File::open(filename)?;
    match Path::new(filename).extension().and_then(|ext| ext.to_str()) {
        Some("img") | Some("bin") | Some("raw") | None => rawloader::load_with_seek(&mut file),
        Some("hex") => hexloader::load(&mut file),
        ext => Err(LoadError::from_message(format!(
            "Unknown file extension: {:?}",
            ext
        ))),
    }
}

pub static SUPPORTED_FORMATS: [&str; 4] = ["img", "bin", "raw", "hex"];
