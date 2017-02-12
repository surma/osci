use memory::Memory;
use std;

/// Like `/dev/null`.
///
/// The `NullMemory` is always 2^32 bytes large (osci’s memory size),
/// reads `0` everywhere and discards all writes.
///
/// # Examples
/// ```
/// use osciemu::memory::{Memory, NullMemory};
///
/// let mut m = NullMemory::new();
/// assert_eq!(m.get(0), 0);
/// m.set(0, 1);
/// assert_eq!(m.get(0), 0);
/// ```
pub struct NullMemory;

impl NullMemory {
    pub fn new() -> NullMemory {
        NullMemory
    }
}

impl Memory for NullMemory {
    #[inline]
    fn get(&self, _: usize) -> u32 {
        0
    }

    #[inline]
    fn set(&mut self, _: usize, _: u32) {}

    #[inline]
    fn size(&self) -> usize {
        std::u32::MAX as usize
    }
}

#[cfg(test)]
mod tests {
    use memory::Memory;

    #[test]
    fn read() {
        let m = super::NullMemory::new();
        assert_eq!(m.get(0), 0);
        assert_eq!(m.get(123), 0);
        assert_eq!(m.get(u32::max_value() as usize), 0);
    }

    #[test]
    fn write() {
        let mut m = super::NullMemory::new();
        assert_eq!(m.get(0), 0);
        m.set(0, 4);
        assert_eq!(m.get(0), 0);
    }
}
