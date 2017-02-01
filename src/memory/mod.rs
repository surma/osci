//! Composable memory behaviors.
//!
//! The memory of an osci CPU is just a sequential chunk for 2^32 memory cells
//! each 8 bits in size. For emulation purposes, different aspects of the
//! memories behaviors are implement in individual types that can be composed
//! using `MappedMemory`.

pub mod nullmemory;
pub mod slicememory;

/// The `Memory` trait is used to specify how to access individual memory cells.
pub trait Memory {
    /// Gets the value of the memory cell at `addr`.
    fn get(&self, addr: usize) -> u32;
    /// Sets the value of the memory cell at `addr`.
    fn set(&mut self, addr: usize, value: u32);
    /// Returns the size of this memory.
    fn size(&self) -> usize;
}
