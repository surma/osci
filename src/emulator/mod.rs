use super::memory::{self, address, Memory};
use super::memory::mappedmemory::MemoryToken;
use super::instruction::Instruction;

pub struct Emulator {
    image_memory: MemoryToken,
    bios_memory: MemoryToken,
    controls_memory: memory::mappedmemory::MemoryToken,
    pub memory: memory::MappedMemory,
    pub ip: usize,
    bios_mounted: bool,
}

impl Emulator
{
    pub fn new(img: Box<Memory>, bios: Box<Memory>) -> Emulator {
        let mut memory = memory::MappedMemory::new();
        memory.mount(0, Box::new(memory::NullMemory::new()));

        let image_memory = memory.mount(0, img);
        let bios_memory = memory.mount(address::BIOS_START_ADDRESS, bios);

        let controls_memory = Box::new(
            memory::SliceMemory::new(address::MAX_ADDRESS - address::CONTROLS_ADDRESS + 1)
        );
        let controls_memory = memory.mount(address::CONTROLS_ADDRESS, controls_memory);

        Emulator {
            memory,
            image_memory,
            bios_memory,
            controls_memory,
            ip: address::BIOS_START_ADDRESS,
            bios_mounted: true,
        }
    }

    pub fn flag_is_set(&self, flag_idx: usize) -> bool {
        let bit = flag_idx % 32;
        self.memory.get(address::FLAGS_START_ADDRESS + flag_idx / 32) & (1 << bit) != 0
    }

    pub fn get_register(&self, reg_idx: usize) -> u32 {
        self.memory.get(address::REGISTERS_START_ADDRESS + reg_idx)
    }

    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);

        self.check_bios_mount()
    }

    fn check_bios_mount(&mut self) {
        if self.flag_is_set(address::FLAG0_BIOS_DONE) && self.bios_mounted {
            self.memory.unmount(&self.bios_memory);
            self.bios_mounted = false;
        } else if !self.flag_is_set(address::FLAG0_BIOS_DONE) && !self.bios_mounted {
            // self.memory.mount(address::BIOS_START_ADDRESS, &self.bios_memory);
            // self.bios_mounted = true;
        }
    }

    pub fn is_halted(&self) -> bool {
        self.flag_is_set(address::FLAG0_HALTED)
    }
}

#[cfg(test)]
mod tests {
    use memory::{SliceMemory, NullMemory, address, Memory};

    #[test]
    fn unmounts_bios() {
        let bios = SliceMemory::from_slice(Box::new(
                                               [address::BIOS_START_ADDRESS as u32 + 4,
                                                 address::BIOS_START_ADDRESS as u32 + 5,
                                                 address::FLAGS_START_ADDRESS as u32,
                                                 0,

                                                 2,
                                                 0,
                                                 0,
                                                 0]));
        let mut emu = super::Emulator::new(Box::new(NullMemory::new()), Box::new(bios));

        assert!(!emu.flag_is_set(address::FLAG0_BIOS_DONE));
        assert_eq!(emu.memory.get(address::BIOS_START_ADDRESS + 4), 2);
        emu.step();
        assert!(emu.flag_is_set(address::FLAG0_BIOS_DONE));
        assert_eq!(emu.memory.get(address::BIOS_START_ADDRESS + 4), 0);
    }

    #[test]
    fn is_halted() {
        let bios = SliceMemory::from_slice(Box::new(
                                               [address::BIOS_START_ADDRESS as u32 + 4,
                                                 address::BIOS_START_ADDRESS as u32 + 5,
                                                 address::FLAGS_START_ADDRESS as u32,
                                                 0,

                                                 1,
                                                 0,
                                                 0,
                                                 0]));
        let mut emu = super::Emulator::new(Box::new(NullMemory::new()), Box::new(bios));

        assert!(!emu.is_halted());
        emu.step();
        assert!(emu.is_halted());
    }

    #[test]
    fn get_register() {
        let mut emu = super::Emulator::new(Box::new(NullMemory::new()), Box::new(NullMemory::new()));
        emu.memory.set(address::REGISTERS_START_ADDRESS + 1, 101);
        emu.memory.set(address::REGISTERS_START_ADDRESS, 100);
        assert_eq!(emu.get_register(1), 101);
        assert_eq!(emu.get_register(0), 100);
    }
}
