//! Various utils to work with osci.
use std::fs::File;
use std::path::Path;
use memory::Memory;
use loader::{hexloader, rawloader, LoadError, Result};

/// Turn a file into a memory.
///
/// `load_file` recognizes multiple file formats. A file type is recognized by its file extension and mapped to one of the implemented loaders.
///
/// - “.raw”, “.bin”, “.img” or no extension: `rawloader`
/// - “.hex”: `hexloader`
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

/// List of formats supported by `load_file`.
///
/// The list contains file extensions that are recognized by `load_file`.
pub static SUPPORTED_FORMATS: [&str; 4] = ["img", "bin", "raw", "hex"];
