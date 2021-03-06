//! Like `/dev/null`.
use std;

use memory::Memory;

/// A read-only memory full of zeros.
///
/// The `NullMemory` is always 2<sup>31</sup> bytes large (osci’s memory size),
/// reads `0` everywhere and discards all writes.
///
/// # Examples
///
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
    fn get(&self, _: usize) -> i32 {
        0
    }

    #[inline]
    fn set(&mut self, _: usize, _: i32) {}

    #[inline]
    fn size(&self) -> usize {
        std::i32::MAX as usize
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
    }

    #[test]
    fn write() {
        let mut m = super::NullMemory::new();
        assert_eq!(m.get(0), 0);
        m.set(0, 4);
        assert_eq!(m.get(0), 0);
    }
}
