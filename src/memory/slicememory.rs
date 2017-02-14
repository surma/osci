use memory::Memory;
use std::boxed::Box;
use std::vec::Vec;
use std::cmp::min;

/// Simple slice-based memory implementation.
///
/// # Examples
///
/// ```
/// use osciemu::memory::Memory;
/// use osciemu::memory::SliceMemory;
///
/// let mut m = SliceMemory::new(16);
/// assert_eq!(m.get(0), 0);
/// m.set(8, 123);
/// assert_eq!(m.get(8), 123);
/// ```
///
/// To construct a `SliceMemory` from a `Vec`, use `into_boxed_slice()`.
///
/// ```
/// # use osciemu::memory::{Memory, SliceMemory};
///
/// let v : Vec<u8> = vec![1, 2, 3, 4];
/// let m = SliceMemory(v.into_boxed_slice());
/// assert_eq!(m.get(0), 0x04030201);
/// ```
///
/// # Panics
/// Just like a slice, `SliceMemory` panics when accessed out of bounds.
///
/// ```
/// # use std::panic;
/// # use osciemu::memory::{Memory, SliceMemory};
/// let mut m = SliceMemory::new(16);
/// let result = panic::catch_unwind(|| {
///     m.get(20); // Out-of-bounds!
/// });
/// assert!(result.is_err());
/// ```
pub struct SliceMemory(pub Box<[u8]>);

impl SliceMemory {
    /// Creates a new `SliceMemory`.
    ///
    /// # Panics
    /// Panics when `size` is not a multiple of the word size.
    pub fn new(size: usize) -> SliceMemory {
        assert!(size % 4 == 0, "Size needs to be a multiple of word size");
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0)
        }
        SliceMemory(v.into_boxed_slice())
    }

    /// Initializes the new memory with the given `u8` slice data.
    ///
    /// # Panics
    /// Panics when `size` is not a multiple of the word size.
    pub fn from_slice(size: usize, data: &[u8]) -> SliceMemory {
        let mut sm = SliceMemory::new(size);
        for i in 0..min(sm.0.len(), data.len()) {
            sm.0[i] = data[i];
        }
        sm
    }

    /// Initializes the new memory with the given `u32` slice data.
    ///
    /// # Panics
    /// Panics when `size` is not a multiple of the word size.
    pub fn from_slice_u32(size: usize, data: &[u32]) -> SliceMemory {
        let mut sm = SliceMemory::new(size);
        for i in 0..min(sm.0.len() >> 2, data.len()) {
            sm.0[i * 4 + 0] = ((data[i] >> 0) & 0xFF) as u8;
            sm.0[i * 4 + 1] = ((data[i] >> 8) & 0xFF) as u8;
            sm.0[i * 4 + 2] = ((data[i] >> 16) & 0xFF) as u8;
            sm.0[i * 4 + 3] = ((data[i] >> 24) & 0xFF) as u8;
        }
        sm
    }
}

impl Memory for SliceMemory {
    fn get(&self, addr: usize) -> u32 {
        assert!(addr % 4 == 0, "Can only read from word boundaries");
        (self.0[addr + 0] as u32) << 0 | (self.0[addr + 1] as u32) << 8 |
        (self.0[addr + 2] as u32) << 16 | (self.0[addr + 3] as u32) << 24
    }
    fn set(&mut self, addr: usize, v: u32) {
        assert!(addr % 4 == 0, "Can only read from word boundaries");
        self.0[addr + 0] = ((v >> 0) & 0xFF) as u8;
        self.0[addr + 1] = ((v >> 8) & 0xFF) as u8;
        self.0[addr + 2] = ((v >> 16) & 0xFF) as u8;
        self.0[addr + 3] = ((v >> 24) & 0xFF) as u8;
    }
    fn size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use memory::Memory;

    #[test]
    fn get_and_set() {
        let mut m = super::SliceMemory::new(16);
        for i in 0..4 {
            m.set(i << 2, i as u32);
        }
        for i in 0..4 {
            assert_eq!(m.get(i << 2), i as u32);
        }
    }

    #[test]
    fn from_slice() {
        let m1 = super::SliceMemory::from_slice(8, &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(m1.get(0), 0x03020100);
        assert_eq!(m1.get(4), 0x07060504);

        let m2 = super::SliceMemory::from_slice(8, &[3, 2, 1, 0, 7, 6, 5, 4]);
        assert_eq!(m2.get(0), 0x00010203);
        assert_eq!(m2.get(4), 0x04050607);
    }

    #[test]
    fn from_slice_u32() {
        let m1 = super::SliceMemory::from_slice_u32(8, &[0xFF00FF00, 0x00FF00FF]);
        assert_eq!(m1.get(0), 0xFF00FF00);
        assert_eq!(m1.get(4), 0x00FF00FF);
    }

    #[test]
    fn size() {
        let m1 = super::SliceMemory::new(16);
        assert_eq!(m1.size(), 16);

        let m2 = super::SliceMemory::from_slice(4, &[1, 2, 3, 4]);
        assert_eq!(m2.size(), 4);
    }
}
