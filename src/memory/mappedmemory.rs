use memory::Memory;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

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
/// let m1 = SliceMemory::from_slice(4, &[1]);
/// let m2 = SliceMemory::from_slice(8, &[2, 2]);
/// let mut mm = MappedMemory::new();
/// mm.mount(0, m1);
/// mm.mount(8, m2);
/// // Now mm =~ [1, _, 2, 2]
/// assert_eq!(mm.get(0), 1);
/// assert_eq!(mm.get(12), 2);
/// ```
///
/// `mount()` returns a `MountToken` that can be used to access the memory
/// similar to `std::cell::RefCell`.
///
/// ```
/// use osciemu::memory::{Memory, SliceMemory, MappedMemory};
///
/// let m1 = SliceMemory::from_slice(4, &[1, 2, 3, 4]);
/// let mut mm = MappedMemory::new();
/// let mt1 = mm.mount(0, m1);
/// mm.set(0, 99);
/// assert_eq!(mt1.borrow().get(0), 99);
/// ```
///
/// # Panics
/// `MappedMemory` panics when an unmapped address is read or written.
pub struct MappedMemory(Vec<Entry>);

struct Entry {
    start_address: usize,
    size: usize,
    memory: Rc<RefCell<Memory>>,
}

/// Represents a mounted `Memory`.
pub struct MountToken {
    memory: Rc<RefCell<Memory>>,
}

impl MountToken {
    /// Borrows a reference to the mounted memory until the `Ref` is destroyed.
    /// See `std::cell::RefCell::borrow()`
    pub fn borrow(&self) -> Ref<Memory> {
        self.memory.borrow()
    }

    /// Borrows a mutable reference to the mounted memory until the `RefMut`
    /// is destroyed.
    /// See `std::cell::RefCell::borrow_mut()`
    pub fn borrow_mut(&self) -> RefMut<Memory> {
        self.memory.borrow_mut()
    }
}

impl MappedMemory {
    pub fn new() -> MappedMemory {
        MappedMemory(Vec::new())
    }

    /// Mounts a `Memory` at the given address.
    ///
    /// # Panics
    /// `mount` panics if a mount is not on a word boundary.
    pub fn mount<T: 'static>(&mut self, start: usize, memory: T) -> MountToken
            where T : Memory {
        assert!(start % 4 == 0, "Mount needs to be on a word boundary");
        let size = memory.size();
        let wrapped_mem = Rc::new(RefCell::new(memory));
        let new_entry = Entry {
            start_address: start,
            size: size,
            memory: wrapped_mem.clone(),
        };
        self.0.push(new_entry);
        MountToken {
            memory: wrapped_mem,
        }
    }

    // pub fn unmount(&mut self, mount_token: MountToken) {
    //     self.0
    //         .iter()
    //         .find(|entry| entry.memory.eq(mount_token.memory))
    //         .map(|mem)
    // }

    fn memory_at_addr(&self, addr: usize) -> Option<&Entry> {
        self.0
            .iter()
            .rev()
            .find(|entry| entry.start_address <= addr && entry.start_address + entry.size > addr)
    }
}

impl Memory for MappedMemory {
    fn get(&self, addr: usize) -> u32 {
        self.memory_at_addr(addr)
            .map(|entry| entry.memory.borrow().get(addr - entry.start_address))
            .expect("Out of bounds")
    }

    fn set(&mut self, addr: usize, value: u32) {
        self.memory_at_addr(addr)
            .map(|entry| entry.memory.borrow_mut().set(addr - entry.start_address, value))
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
        let m1 = SliceMemory::from_slice(4, &[1]);
        let m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, m1);
        mm.mount(8, m2);

        assert!(mm.memory_at_addr(0)
            .map_or(false, |entry| entry.memory.borrow().get(0) == 1));
        assert!(mm.memory_at_addr(4).is_none());
        assert!(mm.memory_at_addr(8)
            .map_or(false, |entry| entry.memory.borrow().get(0) == 2));
        assert!(mm.memory_at_addr(12)
            .map_or(false, |entry| entry.memory.borrow().get(0) == 2));
        assert!(mm.memory_at_addr(16).is_none());
    }

    #[test]
    fn overlapping_mounts() {
        let m1 = NullMemory::new();
        let m2 = SliceMemory::from_slice(8, &[2, 2]);
        let m3 = SliceMemory::from_slice(4, &[3]);
        let mut mm = super::MappedMemory::new();
        mm.mount(0, m1);
        mm.mount(4, m2);
        mm.mount(8, m3);
        assert_eq!(mm.get(0), 0);
        assert_eq!(mm.get(4), 2);
        assert_eq!(mm.get(8), 3);
        assert_eq!(mm.get(12), 0);
    }

    #[test]
    fn get_and_set() {
        let m1 = SliceMemory::from_slice(4, &[1]);
        let m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut mm = super::MappedMemory::new();

        let mt1 = mm.mount(0, m1);
        let mt2 = mm.mount(8, m2);
        assert_eq!(mm.get(0), 1);
        assert_eq!(mm.get(8), 2);

        mm.set(0, 3);
        assert_eq!(mm.get(0), 3);

        mm.set(12, 0);
        assert_eq!(mm.get(12), 0);
        assert_eq!(mt1.borrow().get(0), 3);
        assert_eq!(mt2.borrow().get(4), 0);
    }

    #[test]
    fn size() {
        let m1 = SliceMemory::from_slice(4, &[1]);
        let m2 = SliceMemory::from_slice(8, &[2, 2]);
        let mut mm = super::MappedMemory::new();
        assert_eq!(mm.size(), 0);

        mm.mount(0, m1);
        assert_eq!(mm.size(), 4);

        mm.mount(8, m2);
        assert_eq!(mm.size(), 16);
    }

    #[test]
    fn size_with_overlap() {
        let m1 = SliceMemory::from_slice(20, &[1, 1, 1, 1, 1]);
        let m2 = SliceMemory::from_slice(8, &[2, 2]);
        let m3 = SliceMemory::from_slice(12, &[3, 3, 3]);
        let mut mm = super::MappedMemory::new();

        mm.mount(0, m1);
        mm.mount(8, m2);
        assert_eq!(mm.size(), 20);

        mm.mount(24, m3);
        assert_eq!(mm.size(), 36);
    }

    // #[test]
    // fn unmount() {
    //     let m1 = SliceMemory::from_slice(5, &[1, 1, 1, 1, 1]);
    //     let m2 = SliceMemory::from_slice(5, &[2, 2, 2, 2, 2]);
    //     let mut mm = super::MappedMemory::new();
    //     let mt1 = mm.mount(0, m1);
    //     let mt2 = mm.mount(0, m2);

    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 2);
    //     }
    //     mm.unmount(mt2);
    //     for i in 0..5 {
    //         assert_eq!(mm.get(i), 1);
    //     }
    // }
}
