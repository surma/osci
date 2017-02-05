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
/// let mut m1 = SliceMemory::from_slice(4, &[1]);
/// let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
/// let mut mm = MappedMemory::new();
/// mm.mount(0, &mut m1);
/// mm.mount(8, &mut m2);
/// // Now mm =~ [1, _, 2, 2]
/// assert_eq!(mm.get(0), 1);
/// assert_eq!(mm.get(12), 2);
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
    ///
    /// # Panics
    /// `mount` panics if a mount is not on a word boundary.
    pub fn mount(&mut self, start: usize, memory: &'a mut Memory) {
        assert!(start % 4 == 0, "Mount needs to be on a word boundary");
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
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use memory::{Memory, SliceMemory, NullMemory};

    #[test]
    fn memory_at_addr() {
        let mut m1 = SliceMemory::from_slice(4, &[1]);
        let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, &mut m1);
        mm.mount(8, &mut m2);

        assert!(mm.memory_at_addr(0)
            .map_or(false, |entry| entry.memory.get(0) == 1));
        assert!(mm.memory_at_addr(4).is_none());
        assert!(mm.memory_at_addr(8)
            .map_or(false, |entry| entry.memory.get(0) == 2));
        assert!(mm.memory_at_addr(12)
            .map_or(false, |entry| entry.memory.get(0) == 2));
        assert!(mm.memory_at_addr(16).is_none());
    }

    #[test]
    fn overlapping_mounts() {
        let mut m1 = NullMemory::new();
        let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut m3 = SliceMemory::from_slice(4, &[3]);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, &mut m1);
        mm.mount(4, &mut m2);
        mm.mount(8, &mut m3);

        assert_eq!(mm.get(0), 0);
        assert_eq!(mm.get(4), 2);
        assert_eq!(mm.get(8), 3);
        assert_eq!(mm.get(12), 0);
    }

    #[test]
    fn get_and_set() {
        let mut m1 = SliceMemory::from_slice(4, &[1]);
        let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
        {
            let mut mm = super::MappedMemory::new();
            mm.mount(0, &mut m1);
            mm.mount(8, &mut m2);

            assert_eq!(mm.get(0), 1);
            assert_eq!(mm.get(8), 2);

            mm.set(0, 3);
            assert_eq!(mm.get(0), 3);
            mm.set(12, 0);
            assert_eq!(mm.get(12), 0);
        }
        assert_eq!(m1.get(0), 3);
        assert_eq!(m2.get(4), 0);
    }

    #[test]
    fn size() {
        let mut m1 = SliceMemory::from_slice(4, &[1]);
        let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut mm = super::MappedMemory::new();
        assert_eq!(mm.size(), 0);

        mm.mount(0, &mut m1);
        assert_eq!(mm.size(), 4);

        mm.mount(8, &mut m2);
        assert_eq!(mm.size(), 16);
    }

    #[test]
    fn size_with_overlap() {
        let mut m1 = SliceMemory::from_slice(20, &[1, 1, 1, 1, 1]);
        let mut m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut m3 = SliceMemory::from_slice(12, &[3, 3, 3]);
        let mut mm = super::MappedMemory::new();

        mm.mount(0, &mut m1);
        mm.mount(8, &mut m2);
        assert_eq!(mm.size(), 20);

        mm.mount(24, &mut m3);
        assert_eq!(mm.size(), 36);
    }
}
