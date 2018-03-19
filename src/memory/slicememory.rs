use memory::Memory;
use std::boxed::Box;
use std::vec::Vec;

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
/// let v : Vec<u32> = vec![0, 0xFFFFFFFF];
/// let m = SliceMemory(v.into_boxed_slice());
/// assert_eq!(m.get(0), 0);
/// assert_eq!(m.get(1), 0xFFFFFFFF);
/// ```
pub struct SliceMemory(pub Box<[u32]>);

impl SliceMemory {
    /// Creates a new `SliceMemory`.
    pub fn new(size: usize) -> SliceMemory {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0)
        }
        SliceMemory(v.into_boxed_slice())
    }

    /// Initializes the new memory with the given `u32` slice data.
    pub fn from_slice(data: Box<[u32]>) -> SliceMemory {
        SliceMemory(data)
    }
}

impl Memory for SliceMemory {
    fn get(&self, addr: usize) -> u32 {
        self.0[addr]
    }

    fn set(&mut self, addr: usize, v: u32) {
        self.0[addr] = v;
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
            m.set(i, i as u32);
        }
        for i in 0..4 {
            assert_eq!(m.get(i), i as u32);
        }
    }

    #[test]
    fn from_slice() {
        let s = Box::new([0, 1, 2, 3, 4, 5, 6, 7]);
        let m = super::SliceMemory::from_slice(s);
        assert_eq!(m.get(0), 0);
        assert_eq!(m.get(4), 4);
    }

    #[test]
    fn size() {
        let m = super::SliceMemory::new(16);
        assert_eq!(m.size(), 16);

        let m = super::SliceMemory::from_slice(Box::new([1, 2, 3, 4]));
        assert_eq!(m.size(), 4);
    }
}
