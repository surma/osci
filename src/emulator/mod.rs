use super::memory::{self, addresses, Memory};
use super::memory::mappedmemory::MemoryToken;
use super::instruction::Instruction;

pub struct Emulator<T, U>
    where T: Memory,
          U: Memory
{
    image_memory: MemoryToken<T>,
    bios_memory: MemoryToken<U>,
    controls_memory: memory::mappedmemory::MemoryToken<memory::SliceMemory>,
    pub memory: memory::MappedMemory,
    pub ip: usize,
    bios_mounted: bool,
}

impl<T, U> Emulator<T, U>
    where T: 'static + Memory,
          U: 'static + Memory
{
    pub fn new(img: T, bios: U) -> Emulator<T, U> {
        let mut memory = memory::MappedMemory::new();
        let nm = MemoryToken::new(memory::NullMemory::new());
        memory.mount(0, &nm);

        let bios_memory = memory::mappedmemory::MemoryToken::new(bios);
        let image_memory = memory::mappedmemory::MemoryToken::new(img);
        memory.mount(0, &image_memory);
        memory.mount(addresses::BIOS_START_ADDRESS, &bios_memory);

        let controls_memory =
            MemoryToken::new(memory::SliceMemory::new(addresses::MAX_ADDRESS -
                                                      addresses::FLAGS_START_ADDRESS +
                                                      1));
        memory.mount(addresses::FLAGS_START_ADDRESS, &controls_memory);

        Emulator {
            memory: memory,
            image_memory: image_memory,
            bios_memory: bios_memory,
            ip: addresses::BIOS_START_ADDRESS,
            controls_memory: controls_memory,
            bios_mounted: true,
        }
    }

    pub fn flag_is_set(&self, flag_idx: usize) -> bool {
        let bit = flag_idx % 32;
        self.memory.get(addresses::FLAGS_START_ADDRESS + flag_idx / 32) & (1 << bit) != 0
    }

    pub fn get_register(&self, reg_idx: usize) -> u32 {
        self.memory.get(addresses::REGISTERS_START_ADDRESS + reg_idx * 4)
    }

    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);

        self.check_bios_mount()
    }

    fn check_bios_mount(&mut self) {
        if self.flag_is_set(addresses::FLAG0_BIOS_DONE) && self.bios_mounted {
            self.memory.unmount(&self.bios_memory);
            self.bios_mounted = false;
        } else if !self.flag_is_set(addresses::FLAG0_BIOS_DONE) && !self.bios_mounted {
            self.memory.mount(addresses::BIOS_START_ADDRESS, &self.bios_memory);
            self.bios_mounted = true;
        }
    }

    pub fn is_halted(&self) -> bool {
        self.flag_is_set(addresses::FLAG0_HALTED)
    }
}

#[cfg(test)]
mod tests {
    use memory::{SliceMemory, NullMemory, addresses, Memory};

    #[test]
    fn unmounts_bios() {
        let bios = SliceMemory::from_slice_u32(32,
                                               &[addresses::BIOS_START_ADDRESS as u32 + 16,
                                                 addresses::BIOS_START_ADDRESS as u32 + 20,
                                                 addresses::FLAGS_START_ADDRESS as u32,
                                                 0,

                                                 2,
                                                 0,
                                                 0,
                                                 0]);
        let mut emu = super::Emulator::new(NullMemory::new(), bios);

        assert!(!emu.flag_is_set(addresses::FLAG0_BIOS_DONE));
        assert_eq!(emu.memory.get(addresses::BIOS_START_ADDRESS + 16), 2);
        emu.step();
        assert!(emu.flag_is_set(addresses::FLAG0_BIOS_DONE));
        assert_eq!(emu.memory.get(addresses::BIOS_START_ADDRESS + 16), 0);
    }

    #[test]
    fn is_halted() {
        let bios = SliceMemory::from_slice_u32(32,
                                               &[addresses::BIOS_START_ADDRESS as u32 + 16,
                                                 addresses::BIOS_START_ADDRESS as u32 + 20,
                                                 addresses::FLAGS_START_ADDRESS as u32,
                                                 0,

                                                 1,
                                                 0,
                                                 0,
                                                 0]);
        let mut emu = super::Emulator::new(NullMemory::new(), bios);

        assert!(!emu.is_halted());
        emu.step();
        assert!(emu.is_halted());
    }
}
