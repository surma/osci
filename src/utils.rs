use std::io;
use std::fs::File;
use std::path::Path;
use memory::Memory;
use loader::{hexloader, rawloader};

pub fn load_file(filename: &Path) -> Result<Box<Memory>, io::Error> {
    let mut file = File::open(filename)?;
    match Path::new(filename).extension().and_then(|ext| ext.to_str()) {
        Some("img") | Some("bin") | Some("raw") | None =>
            rawloader::load_with_seek(&mut file),
        Some("hex") =>
            hexloader::load(&mut file),
        _ =>
            Err(io::Error::new(io::ErrorKind::Other, "Unknown file extension"))
    }
}

pub static SUPPORTED_FORMATS: [&str; 4] = ["img", "bin", "raw", "hex"];
