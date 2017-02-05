//! Composable memory behaviors.
//!
//! The `memory` module provides multiple implementations of the `Memory` trait,
//! that can be combined to yield the specified behavior of osci’s memory.
//! By themselves, they are not necessarily compliant.
//!
//! # osci’s memory
//! osci’s memory is only addressable at a word boundary and only entire words
//! can be read or written. A word is 4 bytes in little endian. osci always has
//! 2^32 bytes of virtual memory. Not all the memory addresses are necessarily
//! backed by physical memory. Reads from unmapped memory yield 0. Writes
//! to unmapped memory are discarded.
//!
//! At boot, the BIOS memory is mapped to the address space at 2^31 and the
//! instruction pointer (IP) is set to 2^31. The BIOS memory can be unmapped
//! by setting the `bD` flag. BIOS memory is read-only.
//!
//! The end of the address range is an area of mapped memory for control flags,
//! peripherals and interrupts.
//!
//! ```text
//! +---------------------------------------+ Address 0
//! |                 Word 0                |
//! |                 Word 1                |
//! |                   ...                 |
//! +---------------------------------------+ kFlagBoundary
//! |              Flags Word 0             |
//! |                   ...                 |
//! |              Flags Word i             |
//! +---------------------------------------+ kIvtBoundary
//! |               IVT Entry 0             |
//! |                   ...                 |
//! |               IVT Entry j             |
//! +---------------------------------------+
//! |           IVT Return address          |
//! +---------------------------------------+ kRegisterBoundary
//! |                Register 0             |
//! |                   ...                 |
//! |                Register k             |
//! +---------------------------------------+ kMaxAddress = 2^32-1
//! ```
//!
//! Flags Word 0:
//!
//! ```text
//!  MSB                                LSB
//! +---------------------------------------+
//! |    |    |    |    |    |    | bD | H  | Byte 0
//! +---------------------------------------+
//! |                 Unused                |
//! +---------------------------------------+
//! |                 Unused                |
//! +---------------------------------------+
//! |                 Unused                | Byte 3
//! +---------------------------------------+
//! ```
//!
//! Flags:
//!
//! * biosDone (`bD`): Unmaps the BIOS from the address space
//! * halt (`H`): Halts the CPU

mod nullmemory;
mod slicememory;
mod mappedmemory;
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
