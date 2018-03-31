extern crate byteorder;

use memory::{Memory, SliceMemory};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use self::byteorder::{NetworkEndian, ReadBytesExt};

pub fn load_with_seek<U: Read + Seek>(f: &mut U) -> Result<Box<Memory>, Error> {
    let size: usize = (f.seek(SeekFrom::End(0))? as usize) / 4;
    f.seek(SeekFrom::Start(0))?;
    let mut vec = Vec::<i32>::with_capacity(size);
    vec.resize(size, 0);
    let mut slice: Box<[i32]> = vec.into_boxed_slice();
    f.read_i32_into::<NetworkEndian>(slice.as_mut())?;
    Ok(Box::new(SliceMemory::from_slice(slice)))
}

pub fn load<U: Read>(f: &mut U) -> Result<Box<Memory>, Error> {
    let mut vec = Vec::<i32>::new();
    // let mut buf: [u8; 4];
    loop {
        match f.read_i32::<NetworkEndian>() {
            Ok(word) => vec.push(word),
            Err(err) => {
                if err.kind() == ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(err);
            }
        }
    }
    let slice: Box<[i32]> = vec.into_boxed_slice();
    Ok(Box::new(SliceMemory::from_slice(slice)))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn load() {
        let mut bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0xF0, 0xF1, 0xF2, 0xF3];
        let mem = super::load(&mut bytes).unwrap();
        assert_eq!(mem.get(0), 0x00010203);
        assert_eq!(mem.get(1), 0xF0F1F2F3 as u32 as i32);
    }

    #[test]
    fn load_with_seek() {
        let mut bytes: Cursor<&[u8]> =
            Cursor::new(&[0x00, 0x01, 0x02, 0x03, 0xF0, 0xF1, 0xF2, 0xF3]);
        let mem = super::load_with_seek(&mut bytes).unwrap();
        assert_eq!(mem.get(0), 0x00010203);
        assert_eq!(mem.get(1), 0xF0F1F2F3 as u32 as i32);
    }
}
