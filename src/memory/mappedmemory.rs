use memory::Memory;
use std::vec::Vec;

/// Maps multiple `Memory`s into a single address space.
///
/// The `MappedMemory` allows to unify
/// # Examples
/// ```
/// ```
pub struct MappedMemory<'a> (Vec<Entry<'a>>);

pub struct Entry<'a> {
    memory: &'a mut Memory,
    start_address: usize,
    size: usize,
}

impl<'a> MappedMemory<'a> {
    pub fn new() -> MappedMemory<'a> {
        MappedMemory(Vec::new())
    }

    pub fn mount(&mut self, start: usize, memory: &'a mut Memory) {
        let size = memory.size();
        println!("1>> {}", size);
        let new_entry = Entry {
            memory: memory,
            start_address: start,
            size: size,
        };
        let mut insert_idx = self.0.len();
        {
            let mut iter = self.0.iter().enumerate();
            loop {
                if let Some((idx, entry)) = iter.next() {
                    if entry.start_address >= new_entry.start_address {
                        insert_idx = idx - 1;
                        break;
                    }
                } else { break; }
            }
        }
        self.0.insert(insert_idx, new_entry);
    }

    pub fn memory_at_addr(&self, addr: usize) -> Option<&Entry<'a>> {
        self.0.iter().find(
            |entry| entry.start_address <= addr &&
            entry.start_address + entry.size > addr)
    }

    pub fn memory_at_addr_mut(&mut self, addr: usize) -> Option<&mut Entry<'a>> {
        self.0.iter_mut().find(
            |entry| entry.start_address <= addr &&
            entry.start_address + entry.size > addr)
    }

}

impl<'a> Memory for MappedMemory<'a> {
    fn get (&self, addr: usize) -> u32 {
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
        self.0.last()
            .map_or(0, |entry| {
                println!("2>> {} {}", entry.size, entry.start_address);
                entry.size + entry.start_address as usize
            })
    }
}

#[cfg(test)]
mod test {
    use memory::Memory;
    use memory::SliceMemory;

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
}
