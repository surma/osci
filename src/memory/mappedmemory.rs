use memory::Memory;
use std::vec::Vec;
use std::sync::atomic::{AtomicIsize, Ordering};

/// Maps multiple `Memory`s into a single address space.
///
/// The `MappedMemory` allows to unify multiple `Memory`s in one address space.
/// A `Memory` is mounted at a certain address and is from now on responsible
/// for all reads and writes starting at that address (the “mount point”) and
/// ending where the mounted memory ends. The read and write calls for the
/// responsible `Memory` will be given an address _relative_ to e mount point.
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
///
/// ```
/// use osciemu::memory::{Memory, SliceMemory, MappedMemory};
///
/// let mut mm = MappedMemory::new();
/// let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1]))));
/// let m2 = mm.mount(2, Box::new(SliceMemory::from_slice(Box::new([2, 2]))));
/// // Now mm =~ [1, _, 2, 2]
/// assert_eq!(mm.get(0), 1);
/// assert_eq!(mm.get(3), 2);
/// ```
///
/// A call to `mount` returns a `MemoryToken`. These tokens can be used to
/// borrow a `Memory` even if it has been mounted. This works similar to a
/// `std::cell::RefCell`.
///
/// ```
/// use osciemu::memory::{Memory, NullMemory, SliceMemory, MappedMemory};
/// use osciemu::memory::mappedmemory::MemoryToken;
///
/// let mut mm = MappedMemory::new();
/// let m1 = mm.mount(0, Box::new(NullMemory::new()));
/// let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 2, 3, 4]))));
/// mm.set(0, 99);
/// assert_eq!(mm.get(0), 99);
/// assert_eq!(mm.borrow(&m2).get(0), 99);
/// ```
///
/// The `MemoryToken`s are also used to disable a mounted `Memory`. A disabled
/// memory is effectively unmounted, but remains in the ownership of the
/// `MappedMemory` and can be enabled using `enable_mount()`.
///
/// ```
/// # use osciemu::memory::{Memory, NullMemory, SliceMemory, MappedMemory};
/// # use osciemu::memory::mappedmemory::MemoryToken;
/// # let mut mm = MappedMemory::new();
/// # let m1 = mm.mount(0, Box::new(NullMemory::new()));
/// # let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 2, 3, 4]))));
/// # mm.set(0, 99);
/// // ...
/// mm.disable_mount(&m2);
/// assert_eq!(mm.get(0), 0);
/// assert_eq!(mm.borrow(&m2).get(0), 99);
/// ```
///
/// To move a memory out of the `MappedMemory`, a memory can be unmounted:
///
/// ```
/// # use osciemu::memory::{Memory, NullMemory, SliceMemory, MappedMemory};
/// # use osciemu::memory::mappedmemory::MemoryToken;
/// # let mut mm = MappedMemory::new();
/// # let m1 = mm.mount(0, Box::new(NullMemory::new()));
/// # let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 2, 3, 4]))));
/// # mm.set(0, 99);
/// // ...
/// let m2 = mm.unmount(m2);
/// assert_eq!(mm.get(0), 0);
/// assert_eq!(m2.get(0), 99);
/// ```
///
/// # Panics
/// `MappedMemory` panics when an unmapped address is read or written.
pub struct MappedMemory {
    memories: Vec<Entry>,
}

static id_counter: AtomicIsize = AtomicIsize::new(0);

struct Entry {
    id: isize,
    start_address: usize,
    size: usize,
    enabled: bool,
    memory: Box<Memory>,
}

impl Entry {
    fn contains(&self, addr: usize) -> bool {
        self.start_address <= addr && self.start_address + self.size > addr
    }
}

#[derive(Clone)]
pub struct MemoryToken {
    id: isize,
}

impl MappedMemory {
    pub fn new() -> MappedMemory {
        MappedMemory {
            memories: Vec::new(),
        }
    }

    /// Mounts a `Memory` at the given address.
    pub fn mount(&mut self, start_address: usize, memory: Box<Memory>) -> MemoryToken {
        let size = memory.size();
        let id = id_counter.fetch_add(1, Ordering::Relaxed);
        let new_entry = Entry {
            id,
            start_address,
            size,
            memory,
            enabled: true
        };
        self.memories.push(new_entry);
        MemoryToken{id}
    }

    /// Unmounts a memory.
    pub fn unmount(&mut self, token: MemoryToken) -> Box<Memory> {
        let idx = self.memories
            .iter()
            .enumerate()
            .find(|&(_idx, ref entry)| entry.id == token.id)
            .map(|(idx, ref _entry)| idx)
            .unwrap();

        self.memories.remove(idx).memory
    }

    pub fn is_enabled_mount(&self, token: &MemoryToken) -> bool {
        self.entry_for_token(token).enabled
    }

    /// Disables a memory. This is the same as unmounting without moving
    /// ownership
    pub fn disable_mount(&mut self, token: &MemoryToken) {
        let entry = self.entry_for_token_mut(token);
        entry.enabled = false;
    }

    // Enables a memory. The mount point remains unchanged.
    pub fn enable_mount(&mut self, token: &MemoryToken) {
        let entry = self.entry_for_token_mut(token);
        entry.enabled = true;
    }

    /// Borrows a memory.
    pub fn borrow(&self, token: &MemoryToken) -> &Box<Memory> {
        &self.entry_for_token(token).memory
    }

    /// Mutable borrows a memory.
    pub fn borrow_mut(&mut self, token: &MemoryToken) -> &mut Box<Memory> {
        &mut self.entry_for_token_mut(token).memory
    }

    fn entry_for_token(&self, token: &MemoryToken) -> &Entry {
        self.memories
            .iter()
            .find(|ref entry| entry.id == token.id)
            .unwrap()
    }

    fn entry_for_token_mut(&mut self, token: &MemoryToken) -> &mut Entry {
        self.memories
            .iter_mut()
            .find(|ref mut entry| entry.id == token.id)
            .unwrap()
    }

    fn enabled_entry_at_addr(&self, addr: usize) -> Option<&Entry> {
        self.memories
            .iter()
            .rev()
            .filter(|entry| entry.enabled)
            .find(|entry| entry.contains(addr))
    }

    fn enabled_entry_at_addr_mut(&mut self, addr: usize) -> Option<&mut Entry> {
        self.memories
            .iter_mut()
            .rev()
            .filter(|entry| entry.enabled)
            .find(|entry| entry.contains(addr))
    }
}

impl Memory for MappedMemory {
    fn get(&self, addr: usize) -> u32 {
        self.enabled_entry_at_addr(addr)
            .map(|entry| entry.memory.get(addr - entry.start_address))
            .expect("Out of bounds")
    }

    fn set(&mut self, addr: usize, value: u32) {
        self.enabled_entry_at_addr_mut(addr)
            .map(|entry| entry.memory.set(addr - entry.start_address, value))
            .expect("Out of bounds")
    }

    fn size(&self) -> usize {
        self.memories
            .iter()
            .filter(|entry| entry.enabled)
            .map(|entry| entry.start_address + entry.size)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory, NullMemory};

    #[test]
    fn overlapping_mounts() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(NullMemory::new()));
        let m2 = mm.mount(1, Box::new(SliceMemory::from_slice(Box::new([2, 2]))));
        let m3 = mm.mount(2, Box::new(SliceMemory::from_slice(Box::new([3]))));
        assert_eq!(mm.get(0), 0);
        assert_eq!(mm.get(1), 2);
        assert_eq!(mm.get(2), 3);
        assert_eq!(mm.get(3), 0);
    }

    #[test]
    fn get_and_set() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1]))));
        let m2 = mm.mount(2, Box::new(SliceMemory::from_slice(Box::new([2, 2]))));

        assert_eq!(mm.get(0), 1);
        assert_eq!(mm.get(2), 2);

        mm.set(0, 3);
        assert_eq!(mm.get(0), 3);

        mm.set(3, 0);
        assert_eq!(mm.get(3), 0);
        assert_eq!(mm.borrow(&m1).get(0), 3);
        assert_eq!(mm.borrow(&m2).get(1), 0);
    }

    #[test]
    fn size() {
        let mut mm = super::MappedMemory::new();
        let m1 = Box::new(SliceMemory::from_slice(Box::new([1])));
        let m2 = Box::new(SliceMemory::from_slice(Box::new([2, 2])));
        assert_eq!(mm.size(), 0);

        mm.mount(0, m1);
        assert_eq!(mm.size(), 1);

        mm.mount(2, m2);
        assert_eq!(mm.size(), 4);
    }

    #[test]
    fn size_with_overlap() {
        let mut mm = super::MappedMemory::new();
        let m1 = Box::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1])));
        let m2 = Box::new(SliceMemory::from_slice(Box::new([2, 2])));
        let m3 = Box::new(SliceMemory::from_slice(Box::new([3, 3, 3])));

        mm.mount(0, m1);
        mm.mount(2, m2);
        assert_eq!(mm.size(), 5);

        mm.mount(6, m3);
        assert_eq!(mm.size(), 9);
    }

    #[test]
    fn disable_mount() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1]))));
        let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([2, 2, 2, 2, 2]))));
        let m3 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([3, 3, 3, 3, 3]))));

        for i in 0..5 {
            assert_eq!(mm.get(i), 3);
        }
        mm.disable_mount(&m3);
        for i in 0..5 {
            assert_eq!(mm.get(i), 2);
        }
        mm.disable_mount(&m1);
        for i in 0..5 {
            assert_eq!(mm.get(i), 2);
        }
    }

    #[test]
    fn enable_mount() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1]))));
        let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([2, 2, 2, 2, 2]))));

        for i in 0..5 {
            assert_eq!(mm.get(i), 2);
        }
        mm.disable_mount(&m2);
        for i in 0..5 {
            assert_eq!(mm.get(i), 1);
        }
        mm.enable_mount(&m2);
        for i in 0..5 {
            assert_eq!(mm.get(i), 2);
        }
    }

    #[test]
    fn unmount() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1]))));
        let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([2, 2, 2, 2, 2]))));

        for i in 0..5 {
            assert_eq!(mm.get(i), 2);
        }
        let m2 = mm.unmount(m2);
        for i in 0..5 {
            assert_eq!(mm.get(i), 1);
            assert_eq!(m2.get(i), 2);
        }
    }

    #[test]
    fn is_enabled_mount() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1]))));
        let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([2, 2, 2, 2, 2]))));

        assert!(mm.is_enabled_mount(&m2));
        mm.disable_mount(&m2);
        assert!(!mm.is_enabled_mount(&m2));
        mm.enable_mount(&m2);
        assert!(mm.is_enabled_mount(&m2));
    }

    #[test]
    #[should_panic]
    fn read_disabled() {
        let mut mm = super::MappedMemory::new();
        let m1 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1]))));
        let m2 = mm.mount(1, Box::new(SliceMemory::from_slice(Box::new([2]))));
        let m3 = mm.mount(2, Box::new(SliceMemory::from_slice(Box::new([3]))));
        mm.disable_mount(&m2);
        mm.get(1);
    }
}
