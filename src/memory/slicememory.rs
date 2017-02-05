use memory::Memory;
use std::boxed::Box;
use std::vec::Vec;
use std::cmp::min;

/// Simple slice-based memory implementation.
///
/// # Examples
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
pub struct SliceMemory(Box<[u32]>);

impl SliceMemory {
    /// Creates a new `SliceMemory`.
    ///
    /// # Panics
    /// Panics when `size` is not a multiple of the word size.
    pub fn new(size: usize) -> SliceMemory {
        assert!(size % 4 == 0, "Size needs to be a multiple of word size");
        let mut v = Vec::with_capacity(size >> 2);
        for _ in 0..size >> 2 {
            v.push(0)
        }
        SliceMemory(v.into_boxed_slice())
    }

    /// Initializes the new memory with the given slice data.
    ///
    /// # Panics
    /// Panics when `size` is not a multiple of the word size.
    pub fn from_slice(size: usize, data: &[u32]) -> SliceMemory {
        let mut sm = SliceMemory::new(size);
        for i in 0..min(size >> 2, data.len()) {
            sm.set(i << 2, data[i])
        }
        sm
    }
}

impl Memory for SliceMemory {
    fn get(&self, addr: usize) -> u32 {
        self.0[addr >> 2]
    }
    fn set(&mut self, addr: usize, v: u32) {
        self.0[addr >> 2] = v;
    }
    fn size(&self) -> usize {
        self.0.len() * 4
    }
}

#[cfg(test)]
mod test {
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
        let m1 = super::SliceMemory::from_slice(40, &[0, 1, 2, 3, 4]);
        for i in 0..10 {
            if i < 5 {
                assert_eq!(m1.get(i << 2), i as u32);
            } else {
                assert_eq!(m1.get(i << 2), 0);
            }
        }

        let m2 = super::SliceMemory::from_slice(4, &[1, 2, 3]);
        assert_eq!(m2.get(0), 1);
    }

    #[test]
    fn size() {
        let m1 = super::SliceMemory::new(16);
        assert_eq!(m1.size(), 16);

        let m2 = super::SliceMemory::from_slice(4, &[1, 2, 3, 4]);
        assert_eq!(m2.size(), 4);
    }
}
