//! Composable memory behaviors.
//!
//! The `memory` module provides multiple implementations of the `Memory` trait,
//! that can be combined to yield the specified behavior of osci’s memory.
//! By themselves, they are not necessarily compliant.
//!
//! # osci’s memory
//! osci’s memory is a sequence of word-sized memory cells. One word has 32
//! bits. osci always has 2^31 words of virtual memory. Not all the memory
//! address are necessarily backed by physical memory. Reads from unmapped
//! memory yield 0. Writes to unmapped memory are discarded.
//!
//! At boot, the BIOS memory is mapped to the address space at 2^30 and the
//! instruction pointer (IP) is set to 2^30. The BIOS memory can be unmapped
//! by setting the `bD` flag. BIOS memory is read-only.
//!
//! The end of the address range is an area of mapped memory for control flags,
//! peripherals and interrupts.
//!
//! ```text
//!   +---------------------------------------+ Address 0
//!   |                 Word 0                |
//!   |                 Word 1                |
//!   |                   ...                 |
//!   +---------------------------------------+ STACK_POINTER_ADDRESS
//!   |              Stack Pointer            |
//!   +---------------------------------------+ REGISTERS_START_ADDRESS
//!   |                Register 0             |
//!   |                   ...                 |
//!   |                Register k             |
//!   +---------------------------------------+ IVT_START_ADDRESS
//!   |               IVT Entry 0             |
//!   |                   ...                 |
//!   |               IVT Entry j             |
//!   +---------------------------------------+ FLAGS_START_ADDRESS
//!   |              Flags Word 0             |
//!   |                   ...                 |
//!   |              Flags Word i             |
//!   +---------------------------------------+ MAX_ADDRESS = 2^31-1
//! ```
//!
//! Registers:
//! - Register 0: General purpose
//! - Register 1: General purpose
//! - Register 2: General purpose
//! - Register 3: General purpose
//!
//! IVTs:
//! - IVT Entry 0: Unused
//!
//! Flags Word 0::
//!
//!   MSB                                   LSB
//!   +---------------------------------------+
//!   |    |    |    |    |    |    | bD | H  | Byte 0
//!   +---------------------------------------+
//!   |                 Unused                |
//!   +---------------------------------------+
//!   |                 Unused                |
//!   +---------------------------------------+
//!   |                 Unused                | Byte 3
//!   +---------------------------------------+
//!
//! Flags:
//!
//! * ``biosDone`` (``bD``): Unmaps the BIOS from the address space
//! * ``halt`` (``H``): Halts the CPU

mod nullmemory;
mod slicememory;
pub mod mappedmemory;
pub mod address;
pub mod hookmemory;
pub use self::hookmemory::HookMemory;
pub use self::nullmemory::NullMemory;
pub use self::slicememory::SliceMemory;
pub use self::mappedmemory::MappedMemory;

/// Access to individual memory cells.
pub trait Memory {
    /// Gets the value of the memory cell at `addr`.
    fn get(&self, addr: usize) -> u32;
    /// Sets the value of the memory cell at `addr`.
    fn set(&mut self, addr: usize, value: u32);
    /// Returns the size of this memory in bytes.
    fn size(&self) -> usize;
}
