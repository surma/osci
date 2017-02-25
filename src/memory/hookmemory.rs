use std::boxed::Box;
use memory::Memory;

/// Hook for changing the address before a value is read from `Memory`.
type ReadPreHook = Fn(&mut usize);
/// Hook for changing the value that is returned after being read from `Memory`.
type ReadPostHook = Fn(&usize, &mut u32);
/// Hook for changing both the address and the value before it is written
/// to `Memory`.
type WriteHook = Fn(&mut usize, &mut u32);
/// Hook for changing the reported size of `Memory`.
type SizeHook = Fn(&mut usize);

/// Calls callbacks on reads or writes.
///
/// The `HookMemory` allows to hook into the `Memory` interface. These hooks are
/// allowed to modify the parameters of the call.
///
/// # Examples
///
/// ```
/// use osciemu::memory::{Memory, HookMemory, SliceMemory};
///
/// let mut hook_mem =
///     HookMemory::new(
///         SliceMemory::from_slice_u32(16, &[0, 1, 2, 3])
///     );
/// hook_mem.read_prehook(Box::new(|addr| *addr += 4));
/// hook_mem.read_posthook(Box::new(|addr, val| *val <<= 1));
///
/// // This will read cell 1 and the result will be shifted left by 1.
/// assert_eq!(hook_mem.get(0), 2);
/// ```
///
pub struct HookMemory<T: Memory> {
    get_prehook: Option<Box<ReadPreHook>>,
    get_posthook: Option<Box<ReadPostHook>>,
    set_hook: Option<Box<WriteHook>>,
    size_hook: Option<Box<SizeHook>>,
    memory: T,
}

impl<T: Memory> HookMemory<T> {
    pub fn new(memory: T) -> HookMemory<T> {
        HookMemory {
            get_prehook: None,
            get_posthook: None,
            set_hook: None,
            size_hook: None,
            memory: memory,
        }
    }

    pub fn read_prehook(&mut self, h: Box<ReadPreHook>) {
        self.get_prehook = Some(h);
    }

    pub fn read_posthook(&mut self, h: Box<ReadPostHook>) {
        self.get_posthook = Some(h);
    }

    pub fn write_hook(&mut self, h: Box<WriteHook>) {
        self.set_hook = Some(h);
    }

    pub fn size_hook(&mut self, h: Box<SizeHook>) {
        self.size_hook = Some(h);
    }
}

impl<T: Memory> Memory for HookMemory<T> {
    fn get(&self, mut addr: usize) -> u32 {
        self.get_prehook.as_ref().map(|ref f| f(&mut addr));
        let mut v = self.memory.get(addr);
        self.get_posthook.as_ref().map(|f| f(&addr, &mut v));
        v
    }

    fn set(&mut self, mut addr: usize, mut value: u32) {
        self.set_hook.as_ref().map(|f| f(&mut addr, &mut value));
        self.memory.set(addr, value);
    }

    fn size(&self) -> usize {
        let mut size = self.memory.size();
        self.size_hook.as_ref().map(|f| f(&mut size));
        size
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory};

    #[test]
    fn read_prehook() {
        let mut hook_mem = super::HookMemory::new(SliceMemory::from_slice_u32(12, &[0, 2, 4]));
        assert_eq!(hook_mem.get(0), 0);
        hook_mem.read_prehook(Box::new(|addr| *addr += 4));
        assert_eq!(hook_mem.get(0), 2);
        assert_eq!(hook_mem.get(4), 4);
    }

    #[test]
    fn read_posthook() {
        let mut hook_mem = super::HookMemory::new(SliceMemory::from_slice_u32(12, &[0, 2, 4]));
        assert_eq!(hook_mem.get(0), 0);
        hook_mem.read_posthook(Box::new(|_, val| *val += 1));
        assert_eq!(hook_mem.get(0), 1);
        assert_eq!(hook_mem.get(4), 3);
        assert_eq!(hook_mem.get(8), 5);
    }

    #[test]
    fn write_hook() {
        let mut hook_mem = super::HookMemory::new(SliceMemory::from_slice_u32(8, &[0, 0]));
        assert_eq!(hook_mem.get(0), 0);
        hook_mem.set(0, 1);
        assert_eq!(hook_mem.get(0), 1);
        hook_mem.set(0, 0);
        hook_mem.write_hook(Box::new(|addr, val| {
            *addr += 4;
            *val += 1;
        }));
        hook_mem.set(0, 1);
        assert_eq!(hook_mem.get(0), 0);
        assert_eq!(hook_mem.get(4), 2);
    }

    #[test]
    fn size_hook() {
        let mut hook_mem = super::HookMemory::new(SliceMemory::from_slice_u32(8, &[0, 0]));
        assert_eq!(hook_mem.size(), 8);
        hook_mem.size_hook(Box::new(|size| { *size += 1; }));
        assert_eq!(hook_mem.size(), 9);
    }
}
