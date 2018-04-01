//! Loader for hex text files.
//!
//! This loader can read a simple, text-based hex format and turn it into a `Memory`.
//!
//! # Format
//! The stream can consist of whitespace-separated sequences of hexadecimal characters. If the loader encounteres a `#`, everything until the end-of-line is discarded. This allows comments in the hex file. Hexadecimal numbers may be negative by prefixing them with a minus `-`.
//!
//! # Examples
//!
//! ```
//! # use std::io::Cursor;
//! # use osciemu::loader::hexloader;
//! let mut code = Cursor::new("
//!     DEADBEEF # Numbers are hexadecimal, even without a prefix
//!     1 10 100 # Numbers can have any number of digits
//!     -5 # ... and they can be negative
//! ");
//! let mem = hexloader::load(&mut code).unwrap();
//! assert_eq!(mem.get(0), 0xDEADBEEF);
//! assert_eq!(mem.get(1), 0x1);
//! assert_eq!(mem.get(2), 0x10);
//! assert_eq!(mem.get(3), 0x100);
//! assert_eq!(mem.get(4), -5);
//! ```
use memory::{Memory, SliceMemory};
use std::io::{BufRead, BufReader, Read};
use loader::{LoadError, Result};

pub fn load<U: Read>(f: &mut U) -> Result<Box<Memory>> {
    let mut vec = Vec::<i32>::new();
    let buf = BufReader::new(f);
    for line in buf.lines() {
        let line = line?;
        if line.starts_with("#") {
            continue;
        }
        for chunk in line.split_whitespace() {
            if chunk.len() == 0 {
                continue;
            }
            if chunk.starts_with("#") {
                break;
            }
            if !chunk.chars().all(|c| c.is_digit(16) || c == '-') {
                return Err(LoadError::from_message(format!(
                    "Word contains non-hexnumeric characters: {}",
                    chunk
                )));
            }
            if chunk.starts_with('-') {
                vec.push(i32::from_str_radix(chunk, 16)?);
            } else {
                vec.push(u32::from_str_radix(chunk, 16)? as i32);
            }
        }
    }
    Ok(Box::new(SliceMemory::from_slice(vec.into_boxed_slice())))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn happy_path() {
        let mut code = Cursor::new(
            "
            00010203 70717273 20 103
            ",
        );
        let mem = super::load(&mut code).unwrap();
        assert_eq!(mem.get(0), 0x00010203);
        assert_eq!(mem.get(1), 0x70717273);
        assert_eq!(mem.get(2), 0x20);
        assert_eq!(mem.get(3), 0x103);
    }

    #[test]
    fn hex() {
        let mut code = Cursor::new(
            "
            A a Bb CCc dDd
            ",
        );
        let mem = super::load(&mut code).unwrap();
        assert_eq!(mem.get(0), 0xA);
        assert_eq!(mem.get(1), 0xA);
        assert_eq!(mem.get(2), 0xBB);
        assert_eq!(mem.get(3), 0xCCC);
        assert_eq!(mem.get(4), 0xDDD);
    }

    #[test]
    fn multiline() {
        let mut code = Cursor::new(
            "
            00010203
            70717273
            20
            103
            ",
        );
        let mem = super::load(&mut code).unwrap();
        assert_eq!(mem.get(0), 0x00010203);
        assert_eq!(mem.get(1), 0x70717273);
        assert_eq!(mem.get(2), 0x20);
        assert_eq!(mem.get(3), 0x103);
    }

    #[test]
    fn negative_numbers() {
        let mut code = Cursor::new(
            "
            -1
            -100
            ",
        );
        let mem = super::load(&mut code).unwrap();
        assert_eq!(mem.get(0), -0x1);
        assert_eq!(mem.get(1), -0x100);
    }

    #[test]
    fn comments() {
        let mut code = Cursor::new(
            "
            #lol 00010203
            00010203 #lol
            #70717273
            20
            #oki
            #103
            ",
        );
        let mem = super::load(&mut code).unwrap();
        assert_eq!(mem.get(0), 0x00010203);
        assert_eq!(mem.get(1), 0x20);
    }

    #[test]
    fn invalid_digits() {
        let mut code = Cursor::new(
            "
            0001020g
            ",
        );
        let mem = super::load(&mut code);
        assert!(mem.is_err());
    }

    #[test]
    fn way_too_many_digits() {
        let mut code = Cursor::new(
            "
            0001020123123
            ",
        );
        let mem = super::load(&mut code);
        assert!(mem.is_err());
    }
}
