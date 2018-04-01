//! Composable memory behaviors.
//!
//! The `memory` module provides multiple implementations of the `Memory` trait, that can be combined to implement osci’s memory. By themselves, they are not necessarily compliant to the specification.
//!
//! ## osci’s memory
//! osci’s memory is a sequence of word-sized memory cells. One word has 32 bits. osci always has 2<sup>31</sup> words of virtual memory. Not all the memory address are necessarily mapped to physical memory. Reads from unmapped memory yield 0. Writes to unmapped memory are discarded.
//!
//! ### BIOS
//! At boot, the BIOS memory is mapped to the address space at 2<sup>30</sup>, shadowing the main memory, and the instruction pointer (IP) is set to 2<sup>30</sup>. The BIOS memory can be unmapped by setting the `bD` flag. BIOS memory is read-only.
//!
//! ### Control registers
//! The end of the address range is an area of mapped memory for control flags, peripherals and interrupts.
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
//! The concrete values for these constants can be found in the `address` module.
//!
//! ### Registers
//!
//! - Register 0: General purpose
//! - Register 1: General purpose
//! - Register 2: General purpose
//! - Register 3: General purpose
//!
//! ### IVTs
//!
//! - IVT Entry 0: Unused
//!
//! ### Flags
//! #### Word 0:
//!
//! ```text
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
//! ```
//!
//! Flags:
//!
//! - `biosDone` (`bD`): Unmaps the BIOS from the address space when set to 1
//! - `halt` (`H`): Halts the CPU when set to 1

mod nullmemory;
mod slicememory;
mod readonlymemory;
pub mod mappedmemory;
pub mod address;
pub use self::nullmemory::NullMemory;
pub use self::slicememory::SliceMemory;
pub use self::mappedmemory::MappedMemory;
pub use self::readonlymemory::ReadOnlyMemory;

/// Access to individual memory cells.
pub trait Memory {
    /// Gets the value of the memory cell at `addr`.
    fn get(&self, addr: usize) -> i32;
    /// Sets the value of the memory cell at `addr`.
    fn set(&mut self, addr: usize, value: i32);
    /// Returns the size of this memory in bytes.
    fn size(&self) -> usize;
}
