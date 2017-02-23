use super::memory::{self, addresses};
use super::memory::mappedmemory::MemoryToken;
use super::instruction::Instruction;

use std::cell::{RefCell, RefMut};

pub struct Emulator {
    bios_memory: memory::mappedmemory::MemoryToken<memory::SliceMemory>,
    controls_memory: memory::mappedmemory::MemoryToken<memory::HookMemory<memory::SliceMemory>>,
    pub memory: memory::MappedMemory,
    pub ip: usize,
    halted: RefCell<bool>,
    bios_done: RefCell<bool>,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut memory = memory::MappedMemory::new();
        let nm = MemoryToken::new(memory::NullMemory::new());
        memory.mount(0, &nm);

        let bios_memory = memory::mappedmemory::MemoryToken::new(memory::SliceMemory::new(0));
        let controls_memory =
            MemoryToken::new(
                memory::HookMemory::new(
                    memory::SliceMemory::new(
                        addresses::MAX_ADDRESS - addresses::FLAGS_START_ADDRESS + 1
                    )
                )
            );
        memory.mount(memory::addresses::FLAGS_START_ADDRESS, &controls_memory);
        let mut emulator = Emulator {
            memory: memory,
            bios_memory: bios_memory,
            ip: addresses::BIOS_START_ADDRESS,
            controls_memory: controls_memory,
            halted: RefCell::new(false),
            bios_done: RefCell::new(false),
        };
        let halted = emulator.halted.clone();
        RefMut::map(emulator.controls_memory.borrow_mut(),
                    |r| r as &mut memory::HookMemory<memory::SliceMemory>)
            .write_hook(Box::new(move |addr, val| {
                match *addr {
                    0 => {
                        let flag0_halt = *val & (1 << addresses::FLAG0_HALT) == 1;
                        *halted.borrow_mut() = flag0_halt;

                        // let flag0_bios_done = *val & (1<<addresses::FLAG0_BIOS_DONE) == 1;
                        // if !emulator.bios_done.get() && flag0_bios_done {
                        //     emulator.memory.unmount(&emulator.bios_memory);
                        // } else if emulator.bios_done.get() && !flag0_bios_done {
                        //     // emulator.memory.mount(addresses::BIOS_START_ADDRESS, )
                        // }
                    }
                    _ => {}
                }
            }));
        emulator
    }

    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);
    }

    pub fn is_halted(&self) -> bool {
        *self.halted.borrow()
    }
}
