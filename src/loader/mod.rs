//! Loaders for osci memory images.
use memory::Memory;
use std::io::{Read, Error};

pub mod rawloader;

pub type Loader<T: Memory> = fn (f: Read) -> Result<T, Error>;
