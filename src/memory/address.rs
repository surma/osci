//! Important memory addresses and constants.
//!
//! Most of these constants have been introduced in the diagram in the `memory` module.

/// Highest address in osci’s memory.
pub const MAX_ADDRESS: usize = 0x7FFFFFFF;
/// Number of registers.
pub const NUM_REGISTERS: usize = 4;
/// Number of IVT table entries.
pub const NUM_IVT_ENTRIES: usize = 1;
/// Number of flag words.
pub const NUM_FLAGS: usize = 1;
/// Lowest address of the BIOS memory region.
pub const BIOS_START_ADDRESS: usize = 0x40000000;
/// Lowest address of the flags memory region.
pub const FLAGS_START_ADDRESS: usize = MAX_ADDRESS - NUM_FLAGS;
/// Lowest address of the IVT memory region.
pub const IVT_START_ADDRESS: usize = FLAGS_START_ADDRESS - NUM_IVT_ENTRIES;
/// Lowest address of the registers memory region.
pub const REGISTERS_START_ADDRESS: usize = IVT_START_ADDRESS - NUM_REGISTERS;
/// Address of the stack pointer register.
pub const STACK_POINTER_ADDRESS: usize = REGISTERS_START_ADDRESS - 1;
/// Lowest address of all the control memory regions.
pub const CONTROLS_ADDRESS: usize = STACK_POINTER_ADDRESS;

/// Bit number of the `H` bit in the first flag word.
pub const FLAG0_HALTED: usize = 0;
/// Bit number of the `bD` bit in the first flag word.
pub const FLAG0_BIOS_DONE: usize = 1;
