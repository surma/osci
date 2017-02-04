use memory::Memory;
use std::boxed::Box;
use std::vec::Vec;

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
/// # use osciemu::memory::Memory;
/// # use osciemu::memory::SliceMemory;
/// let mut m = SliceMemory::new(16);
/// let result = panic::catch_unwind(|| {
///     m.get(17); // Out-of-bounds!
/// });
/// assert!(result.is_err());
/// ```
pub struct SliceMemory(Box<[u32]>);

impl SliceMemory {
    pub fn new(size: usize) -> SliceMemory {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0)
        }
        SliceMemory(v.into_boxed_slice())
    }

    // Like `new()` but writes `value` to every cell
    pub fn with_value(size: usize, value: u32) -> SliceMemory {
        let mut sm = SliceMemory::new(size);
        for i in 0..size {
            sm.set(i, value);
        }
        sm
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
mod test {
    use memory::Memory;

    #[test]
    fn get_and_set() {
        let mut m = super::SliceMemory::new(16);
        for i in 0..16 {
            m.set(i, i as u32);
        }
        for i in 0..16 {
            assert_eq!(m.get(i), i as u32);
        }
    }

    #[test]
    fn with_value() {
        let m = super::SliceMemory::with_value(16, 9);
        for i in 0..m.size() {
            assert_eq!(m.get(i), 9);
        }
    }

    #[test]
    fn size() {
        let m1 = super::SliceMemory::new(16);
        assert_eq!(m1.size(), 16);

        let m2 = super::SliceMemory::with_value(16, 1);
        assert_eq!(m2.size(), 16);
    }
}
