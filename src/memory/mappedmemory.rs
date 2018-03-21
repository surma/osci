use memory::Memory;
use std::vec::Vec;
use std::sync::atomic::{AtomicIsize, Ordering};

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
/// The `MemoryToken`s can be used to access the `Memory` even if it has been
/// mounted. They work like `std::cell::RefCell`.
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
/// // assert_eq!(m2.borrow().get(0), 99);
/// ```
///
/// The `MemoryToken`s are also used to unmount a mounted `Memory`.
///
/// ```
/// # use osciemu::memory::{Memory, NullMemory, SliceMemory, MappedMemory};
/// # use osciemu::memory::mappedmemory::MemoryToken;
/// # let mut mm = MappedMemory::new();
/// # let m1 = mm.mount(0, Box::new(NullMemory::new()));
/// # let m2 = mm.mount(0, Box::new(SliceMemory::from_slice(Box::new([1, 2, 3, 4]))));
/// # mm.set(0, 99);
/// // ...
/// // mm.unmount(&m2);
/// // assert_eq!(mm.get(0), 0);
/// // assert_eq!(m2.borrow().get(0), 99);
/// ```
///
/// # Panics
/// `MappedMemory` panics when an unmapped address is read or written.
pub struct MappedMemory {
    mounted_memories: Vec<Entry>,
    unmounted_memories: Vec<Entry>,
}

static id_counter: AtomicIsize = AtomicIsize::new(0);

struct Entry {
    id: isize,
    start_address: usize,
    size: usize,
    memory: Box<Memory>,
}

#[derive(Clone)]
pub struct MemoryToken {
    id: isize,
}

impl MappedMemory {
    pub fn new() -> MappedMemory {
        MappedMemory {
            mounted_memories: Vec::new(),
            unmounted_memories: Vec::new(),
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
        };
        self.mounted_memories.push(new_entry);
        MemoryToken{id}
    }

    /// Unmounts the `Memory` references by the `MemoryToken`. After unmounting,
    /// `MappedMemory` does not hold any references to the `Memory`.
    pub fn unmount(&mut self, token: &MemoryToken) {
        let entry = self.mounted_memories
            .iter()
            .enumerate()
            .find(|&(_, entry)| entry.id == token.id)
            .map(|(idx, _)| idx)
            .map(|idx| {
                self.mounted_memories.remove(idx)
            }).unwrap();
        self.unmounted_memories.push(entry);
    }

    pub fn borrow(&self, token: &MemoryToken) -> &Box<Memory> {
        let idx = self.mounted_memories
            .iter()
            .enumerate()
            .find(|&(idx, entry)| entry.id == token.id)
            .map(|(idx, _)| idx)
            .unwrap();
        &self.mounted_memories.get(idx).unwrap().memory
    }

    pub fn borrow_mut(&mut self, token: &MemoryToken) -> &mut Box<Memory> {
        let idx = self.mounted_memories
            .iter()
            .enumerate()
            .find(|&(idx, entry)| entry.id == token.id)
            .map(|(idx, _)| idx)
            .unwrap();
        &mut self.mounted_memories.get_mut(idx).unwrap().memory
    }

    // fn memory_for_token(&self, token: &MemoryToken): &Entry {
    // }

    fn entry_at_addr(&self, addr: usize) -> Option<&Entry> {
        self.mounted_memories
            .iter()
            .rev()
            .find(|entry| entry.start_address <= addr && entry.start_address + entry.size > addr)
    }

    fn entry_at_addr_mut(&mut self, addr: usize) -> Option<&mut Entry> {
        self.mounted_memories
            .iter_mut()
            .rev()
            .find(|entry| entry.start_address <= addr && entry.start_address + entry.size > addr)
    }
}

impl Memory for MappedMemory {
    fn get(&self, addr: usize) -> u32 {
        self.entry_at_addr(addr)
            .map(|entry| entry.memory.get(addr - entry.start_address))
            .expect("Out of bounds")
    }

    fn set(&mut self, addr: usize, value: u32) {
        self.entry_at_addr_mut(addr)
            .map(|entry| entry.memory.set(addr - entry.start_address, value))
            .expect("Out of bounds")
    }

    fn size(&self) -> usize {
        self.mounted_memories
            .iter()
            .map(|entry| entry.start_address + entry.size)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory, NullMemory};

    // #[test]
    // fn memory_at_addr() {
    //     let m1 = SliceMemory::from_slice(Box::new([1]));
    //     let m2 = SliceMemory::from_slice(Box::new([2, 2]));
    //     let mut mm = super::MappedMemory::new();
    //     let m1 = mm.mount(0, &m1);
    //     let m2 = mm.mount(2, &m2);

    //     assert!(mm.memory_at_addr(0)
    //         .map_or(false, |entry| entry.memory.get(0) == 1));
    //     assert!(mm.memory_at_addr(1).is_none());
    //     assert!(mm.memory_at_addr(2)
    //         .map_or(false, |entry| entry.memory.get(0) == 2));
    //     assert!(mm.memory_at_addr(3)
    //         .map_or(false, |entry| entry.memory.get(0) == 2));
    //     assert!(mm.memory_at_addr(4).is_none());
    // }

    // #[test]
    // fn overlapping_mounts() {
    //     let m1 = super::MemoryToken::new(NullMemory::new());
    //     let m2 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([2, 2])));
    //     let m3 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([3])));
    //     let mut mm = super::MappedMemory::new();
    //     mm.mount(0, &m1);
    //     mm.mount(1, &m2);
    //     mm.mount(2, &m3);
    //     assert_eq!(mm.get(0), 0);
    //     assert_eq!(mm.get(1), 2);
    //     assert_eq!(mm.get(2), 3);
    //     assert_eq!(mm.get(3), 0);
    // }

    // #[test]
    // fn get_and_set() {
    //     let m1 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([1])));
    //     let m2 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([2, 2])));
    //     let mut mm = super::MappedMemory::new();

    //     mm.mount(0, &m1);
    //     mm.mount(2, &m2);
    //     assert_eq!(mm.get(0), 1);
    //     assert_eq!(mm.get(2), 2);

    //     mm.set(0, 3);
    //     assert_eq!(mm.get(0), 3);

    //     mm.set(3, 0);
    //     assert_eq!(mm.get(3), 0);
    //     assert_eq!(m1.borrow().get(0), 3);
    //     assert_eq!(m2.borrow().get(1), 0);
    // }

    // #[test]
    // fn size() {
    //     let m1 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([1])));
    //     let m2 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([2, 2])));
    //     let mut mm = super::MappedMemory::new();
    //     assert_eq!(mm.size(), 0);

    //     mm.mount(0, &m1);
    //     assert_eq!(mm.size(), 1);

    //     mm.mount(2, &m2);
    //     assert_eq!(mm.size(), 4);
    // }

    // #[test]
    // fn size_with_overlap() {
    //     let m1 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1])));
    //     let m2 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([2, 2])));
    //     let m3 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([3, 3, 3])));
    //     let mut mm = super::MappedMemory::new();

    //     mm.mount(0, &m1);
    //     mm.mount(2, &m2);
    //     assert_eq!(mm.size(), 5);

    //     mm.mount(6, &m3);
    //     assert_eq!(mm.size(), 9);
    // }

    // #[test]
    // fn unmount() {
    //     let m1 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([1, 1, 1, 1, 1])));
    //     let m2 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([2, 2, 2, 2, 2])));
    //     let m3 = super::MemoryToken::new(SliceMemory::from_slice(Box::new([3, 3, 3, 3, 3])));
    //     let mut mm = super::MappedMemory::new();
    //     mm.mount(0, &m1);
    //     mm.mount(0, &m2);
    //     mm.mount(0, &m3);

    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 3);
    //     }
    //     mm.unmount(&m3);
    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 2);
    //     }
    //     mm.unmount(&m3);
    //     mm.unmount(&m3);
    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 2);
    //     }
    //     mm.unmount(&m1);
    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 2);
    //     }
    // }
}
