use super::memory::{self, addresses};
use super::instruction::Instruction;

pub struct Emulator {
    bios_memory: memory::mappedmemory::MountToken,
    controls_memory: memory::mappedmemory::MountToken,
    pub memory: memory::MappedMemory,
    pub ip: usize,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut memory = memory::MappedMemory::new();
        memory.mount(0, memory::NullMemory::new());

        let bios_memory = memory::mappedmemory::MountToken::new(memory::NullMemory::new());
        let controls_memory =
            memory::SliceMemory::new(addresses::MAX_ADDRESS - addresses::FLAGS_START_ADDRESS + 1);
        let controls_token = memory.mount(memory::addresses::FLAGS_START_ADDRESS, controls_memory);
        let mut emulator = Emulator {
            memory: memory,
            bios_memory: bios_memory,
            ip: addresses::BIOS_START_ADDRESS,
            controls_memory: controls_token,
        };
        emulator
    }

    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);
    }
}
