//! osci emulator.
//!
//! The `osci` module combines the other modules to form a full-fledged emulator for the osci architecture.
//!
//! # Examples
//!
//! ```
//! # use osciemu::emulator::Emulator;
//! # use osciemu::memory::Memory;
//!
//! let mut bios_code = std::io::Cursor::new("
//!     ## Calculate 0x10 - 0x3 and store it in register 0
//!     40000004 40000005 7FFFFFF9 0
//!     10 3
//! ");
//! let mut bios = osciemu::loader::hexloader::load(&mut bios_code).unwrap();
//! let mut emu = Emulator::from_bios_only(bios);
//! emu.step();
//! assert_eq!(emu.get_register(0), 0x10 - 0x3);
//! ```
use super::memory::{self, address, Memory, SliceMemory};
use super::memory::mappedmemory::MemoryToken;
use super::instruction::Instruction;

// Emulator for osci.
pub struct Emulator {
    bios_memory_token: MemoryToken,
    /// Memory
    pub memory: memory::MappedMemory,
    /// Instruction pointer
    pub ip: usize,
}

impl Emulator {
    /// Initializes an `Emulator` with the given BIOS.
    ///
    /// Equivalent to calling `new()` with an empty `SliceMemory` as the main memory.
    pub fn from_bios_only(bios: Box<Memory>) -> Emulator {
        Emulator::new(bios, Box::new(SliceMemory::new(0)))
    }

    /// Initializes an `Emulator` with the given BIOS and main memory.
    pub fn new(bios: Box<Memory>, img: Box<Memory>) -> Emulator {
        let mut memory = memory::MappedMemory::new();
        memory.mount(0, Box::new(memory::NullMemory::new()));

        memory.mount(0, img);
        let bios_memory_token = memory.mount(
            address::BIOS_START_ADDRESS,
            Box::new(memory::ReadOnlyMemory::new(bios)),
        );

        let controls_memory = Box::new(memory::SliceMemory::new(
            address::MAX_ADDRESS - address::CONTROLS_ADDRESS + 1,
        ));
        memory.mount(address::CONTROLS_ADDRESS, controls_memory);

        Emulator {
            memory,
            bios_memory_token,
            ip: address::BIOS_START_ADDRESS,
        }
    }

    /// Checks if a flag is set.
    ///
    /// Use with the constant from `osciemu::memory::address`.
    pub fn is_flag_set(&self, flag_idx: usize) -> bool {
        let bit = flag_idx % 32;
        self.memory
            .get(address::FLAGS_START_ADDRESS + flag_idx / 32) & (1 << bit) != 0
    }

    /// Gets the current value of the given register.
    pub fn get_register(&self, reg_idx: usize) -> i32 {
        self.memory.get(address::REGISTERS_START_ADDRESS + reg_idx)
    }

    /// Executes one cycle.
    ///
    /// This method will execute a cycle even if the halted flag is set.
    pub fn step(&mut self) {
        let instr = Instruction::from_memory(self.ip, &self.memory);
        instr.execute(&mut self.ip, &mut self.memory);

        self.check_bios_mount()
    }

    fn is_bios_mounted(&self) -> bool {
        self.memory.is_enabled_mount(&self.bios_memory_token)
    }

    fn check_bios_mount(&mut self) {
        if self.is_flag_set(address::FLAG_BIOS_DONE) && self.is_bios_mounted() {
            self.memory.disable_mount(&self.bios_memory_token);
        } else if !self.is_flag_set(address::FLAG_BIOS_DONE) && !self.is_bios_mounted() {
            self.memory.enable_mount(&self.bios_memory_token);
        }
    }

    /// Checks if the halted flag is set.
    ///
    /// Equivalent to calling `is_flag_set(osciemu::memory::address::FLAG_HALTED)`.
    pub fn is_halted(&self) -> bool {
        self.is_flag_set(address::FLAG_HALTED)
    }
}

#[cfg(test)]
mod tests {
    use memory::{address, Memory, NullMemory, SliceMemory};

    #[test]
    fn unmounts_bios() {
        let bios = SliceMemory::from_slice(Box::new([
            address::BIOS_START_ADDRESS as i32 + 4,
            address::BIOS_START_ADDRESS as i32 + 5,
            address::FLAGS_START_ADDRESS as i32,
            0,
            2,
            0,
            0,
            0,
        ]));
        let mut emu = super::Emulator::from_bios_only(Box::new(bios));

        assert!(!emu.is_flag_set(address::FLAG_BIOS_DONE));
        assert_eq!(emu.memory.get(address::BIOS_START_ADDRESS + 4), 2);
        emu.step();
        assert!(emu.is_flag_set(address::FLAG_BIOS_DONE));
        assert_eq!(emu.memory.get(address::BIOS_START_ADDRESS + 4), 0);
    }

    #[test]
    fn is_halted() {
        let bios = SliceMemory::from_slice(Box::new([
            address::BIOS_START_ADDRESS as i32 + 4,
            address::BIOS_START_ADDRESS as i32 + 5,
            address::FLAGS_START_ADDRESS as i32,
            0,
            1,
            0,
            0,
            0,
        ]));
        let mut emu = super::Emulator::from_bios_only(Box::new(bios));

        assert!(!emu.is_halted());
        emu.step();
        assert!(emu.is_halted());
    }

    #[test]
    fn readonly_bios() {
        let bios = SliceMemory::from_slice(Box::new([
            address::BIOS_START_ADDRESS as i32 + 4,
            address::BIOS_START_ADDRESS as i32 + 5,
            address::BIOS_START_ADDRESS as i32 + 6,
            0,
            1,
            0,
            0,
            0,
        ]));
        let mut emu = super::Emulator::from_bios_only(Box::new(bios));
        emu.step();
        assert_eq!(emu.memory.get(address::BIOS_START_ADDRESS + 6), 0);
    }

    #[test]
    fn get_register() {
        let mut emu =
            super::Emulator::from_bios_only(Box::new(NullMemory::new()));
        emu.memory.set(address::REGISTERS_START_ADDRESS + 1, 101);
        emu.memory.set(address::REGISTERS_START_ADDRESS, 100);
        assert_eq!(emu.get_register(1), 101);
        assert_eq!(emu.get_register(0), 100);
    }
}
