pub const NUM_REGISTERS: usize = 4;
pub const MAX_ADDRESS: usize = 0xFFFFFFFF;
pub const NUM_IVT_ENTRIES: usize = 1;
pub const NUM_FLAGS: usize = 1;
pub const REGISTERS_START_ADDRESS: usize = MAX_ADDRESS - 4 * NUM_REGISTERS + 1;
pub const IVT_RETURN_ADDRESS: usize = REGISTERS_START_ADDRESS - 4;
pub const IVT_START_ADDRESS: usize = IVT_RETURN_ADDRESS - 4 * NUM_IVT_ENTRIES;
pub const FLAGS_START_ADDRESS: usize = IVT_START_ADDRESS - 4 * NUM_FLAGS;
