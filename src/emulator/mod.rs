use super::memory::{self, addresses};
use super::memory::mappedmemory::MemoryToken;
use super::instruction::Instruction;

pub struct Emulator {
    bios_memory: memory::mappedmemory::MemoryToken,
    controls_memory: memory::mappedmemory::MemoryToken,
    pub memory: memory::MappedMemory,
    pub ip: usize,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut memory = memory::MappedMemory::new();
        let nm = MemoryToken::new(memory::NullMemory::new());
        memory.mount(0, &nm);

        let bios_memory = memory::mappedmemory::MemoryToken::new(memory::NullMemory::new());
        let controls_memory =
            MemoryToken::new(memory::SliceMemory::new(addresses::MAX_ADDRESS -
                                                      addresses::FLAGS_START_ADDRESS +
                                                      1));
        memory.mount(memory::addresses::FLAGS_START_ADDRESS, &controls_memory);
        let mut emulator = Emulator {
            memory: memory,
            bios_memory: bios_memory,
            ip: addresses::BIOS_START_ADDRESS,
            controls_memory: controls_memory,
        };
        emulator
    }

    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);
    }
}
