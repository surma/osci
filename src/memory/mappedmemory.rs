use memory::Memory;
use std::vec::Vec;

/// Maps multiple `Memory`s into a single address space.
///
/// The `MappedMemory` allows to unify multiple `Memory`s in one address space.
/// A `Memory` is mounted at a certain address and is from now on responsible
/// for all reads and writes between that starting at that address (the “mount
/// point”) and ending where the mounted memory ends.
/// The read and write calls for the responsible `Memory` will be given an
/// address _relative_ to the mount point.
///
/// ```text
///                   Unmapped
///                <-------->
///       mem_a                   mem_b
/// |--------------|            |------|
///                            NullMemory
///                         |--------------
/// |------------- mapped_mem ----------->
/// |              |            |      |
/// 0            0x100        0x200  0x280
/// ```
///
/// For example: `mapped_mem.get(0x208)` would yield the same value as
/// `mem_b.get(0x008)`.
///
/// # Examples
/// ```
/// use osciemu::memory::{Memory, SliceMemory, MappedMemory};
///
/// // m1 =~ [1]
/// let mut m1 = SliceMemory::with_value(1, 1);
/// // m2 =~ [2, 2]
/// let mut m2 = SliceMemory::with_value(2, 2);
/// let mut mm = MappedMemory::new();
/// mm.mount(0, &mut m1);
/// mm.mount(2, &mut m2);
/// // Now mm =~ [1, _, 2, 2]
/// assert_eq!(mm.get(0), 1);
/// assert_eq!(mm.get(3), 2);
/// ```
///
/// # Panics
/// `MappedMemory` panics when an unmapped address is read or written.
pub struct MappedMemory<'a>(Vec<Entry<'a>>);

pub struct Entry<'a> {
    memory: &'a mut Memory,
    start_address: usize,
    size: usize,
}

impl<'a> MappedMemory<'a> {
    pub fn new() -> MappedMemory<'a> {
        MappedMemory(Vec::new())
    }

    /// Mounts a `Memory` at the given address.
    pub fn mount(&mut self, start: usize, memory: &'a mut Memory) {
        let size = memory.size();
        let new_entry = Entry {
            memory: memory,
            start_address: start,
            size: size,
        };
        self.0.push(new_entry);
    }

    fn memory_at_addr(&self, addr: usize) -> Option<&Entry<'a>> {
        self.0
            .iter()
            .rev()
            .find(|entry| entry.start_address <= addr && entry.start_address + entry.size > addr)
    }

    fn memory_at_addr_mut(&mut self, addr: usize) -> Option<&mut Entry<'a>> {
        self.0
            .iter_mut()
            .rev()
            .find(|entry| entry.start_address <= addr && entry.start_address + entry.size > addr)
    }
}

impl<'a> Memory for MappedMemory<'a> {
    fn get(&self, addr: usize) -> u32 {
        self.memory_at_addr(addr)
            .map(|entry| entry.memory.get(addr - entry.start_address))
            .expect("Out of bounds")
    }

    fn set(&mut self, addr: usize, value: u32) {
        self.memory_at_addr_mut(addr)
            .map(|entry| entry.memory.set(addr - entry.start_address, value))
            .expect("Out of bounds")
    }

    fn size(&self) -> usize {
        self.0
            .iter()
            .map(|entry| entry.start_address + entry.size)
            .max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use memory::{Memory, SliceMemory, NullMemory};

    #[test]
    fn memory_at_addr() {
        let mut m1 = SliceMemory::with_value(1, 1);
        let mut m2 = SliceMemory::with_value(2, 2);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, &mut m1);
        mm.mount(2, &mut m2);

        assert!(mm.memory_at_addr(0)
            .map_or(false, |entry| entry.memory.get(0) == 1));
        assert!(mm.memory_at_addr(1).is_none());
        assert!(mm.memory_at_addr(2)
            .map_or(false, |entry| entry.memory.get(0) == 2));
        assert!(mm.memory_at_addr(3)
            .map_or(false, |entry| entry.memory.get(0) == 2));
        assert!(mm.memory_at_addr(4).is_none());
    }

    #[test]
    fn overlapping_mounts() {
        let mut m1 = NullMemory::new();
        let mut m2 = SliceMemory::with_value(2, 2);
        let mut m3 = SliceMemory::with_value(1, 3);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, &mut m1);
        mm.mount(1, &mut m2);
        mm.mount(2, &mut m3);

        assert_eq!(mm.get(0), 0);
        assert_eq!(mm.get(1), 2);
        assert_eq!(mm.get(2), 3);
        assert_eq!(mm.get(3), 0);
    }

    #[test]
    fn get_and_set() {
        let mut m1 = SliceMemory::with_value(1, 1);
        let mut m2 = SliceMemory::with_value(2, 2);
        {
            let mut mm = super::MappedMemory::new();
            mm.mount(0, &mut m1);
            mm.mount(2, &mut m2);

            assert_eq!(mm.get(0), 1);
            assert_eq!(mm.get(2), 2);

            mm.set(0, 3);
            assert_eq!(mm.get(0), 3);
            mm.set(3, 0);
            assert_eq!(mm.get(3), 0);
        }
        assert_eq!(m1.get(0), 3);
        assert_eq!(m2.get(1), 0);
    }

    #[test]
    fn size() {
        let mut m1 = SliceMemory::with_value(1, 1);
        let mut m2 = SliceMemory::with_value(2, 2);
        let mut mm = super::MappedMemory::new();
        assert_eq!(mm.size(), 0);

        mm.mount(0, &mut m1);
        assert_eq!(mm.size(), 1);

        mm.mount(2, &mut m2);
        assert_eq!(mm.size(), 4);
    }

    #[test]
    fn size_with_overlap() {
        let mut m1 = SliceMemory::with_value(10, 1);
        let mut m2 = SliceMemory::with_value(2, 2);
        let mut m3 = SliceMemory::with_value(3, 3);
        let mut mm = super::MappedMemory::new();

        mm.mount(0, &mut m1);
        mm.mount(2, &mut m2);
        assert_eq!(mm.size(), 10);

        mm.mount(9, &mut m3);
        assert_eq!(mm.size(), 12);
    }
}
