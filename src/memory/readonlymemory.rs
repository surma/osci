use memory::Memory;

/// Wraps another `Memory` and discards all writes.
pub struct ReadOnlyMemory(Box<Memory>);

impl ReadOnlyMemory {
    pub fn new(m: Box<Memory>) -> ReadOnlyMemory {
        ReadOnlyMemory(m)
    }
}

impl Memory for ReadOnlyMemory {
    fn get(&self, addr: usize) -> i32 {
        self.0.get(addr)
    }

    #[inline]
    fn set(&mut self, _: usize, _: i32) {}

    #[inline]
    fn size(&self) -> usize {
        self.0.size()
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory};

    #[test]
    fn read() {
        let sm = Box::new(SliceMemory::from_slice(Box::new([0, 1, 2, 3])));
        let m = super::ReadOnlyMemory::new(sm);
        assert_eq!(m.get(0), 0);
        assert_eq!(m.get(3), 3);
    }

    #[test]
    fn write() {
        let mut sm = SliceMemory::from_slice(Box::new([0, 1, 2, 3]));
        let mut m = super::ReadOnlyMemory::new(Box::new(sm));
        m.set(0, 9);
        assert_eq!(m.get(0), 0);
        m.set(3, 4);
        assert_eq!(m.get(3), 3);
    }

    #[test]
    fn size() {
        let sm = SliceMemory::from_slice(Box::new([0, 1, 2, 3]));
        let m = super::ReadOnlyMemory::new(Box::new(sm));
        assert_eq!(m.size(), 4);
    }
}
