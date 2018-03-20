pub const BIOS_START_ADDRESS: usize = 0x40000000;
pub const NUM_REGISTERS: usize = 4;
pub const MAX_ADDRESS: usize = 0x8FFFFFFF;
pub const NUM_IVT_ENTRIES: usize = 1;
pub const NUM_FLAGS: usize = 1;
pub const FLAGS_START_ADDRESS: usize = MAX_ADDRESS - NUM_FLAGS;
pub const IVT_START_ADDRESS: usize = FLAGS_START_ADDRESS - NUM_IVT_ENTRIES;
pub const REGISTERS_START_ADDRESS: usize = IVT_START_ADDRESS - NUM_REGISTERS;
pub const STACK_POINTER_ADDRESS: usize = REGISTERS_START_ADDRESS - 1;
pub const CONTROLS_ADDRESS: usize = STACK_POINTER_ADDRESS;

pub const FLAG0_HALTED: usize = 0;
pub const FLAG0_BIOS_DONE: usize = 1;
