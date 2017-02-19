#[macro_use]
extern crate clap;
extern crate osciemu;

use std::vec::Vec;
use std::fs::File;
use std::io;
use std::io::Read;
use osciemu::memory::{addresses, Memory, SliceMemory};
use osciemu::memory::mappedmemory::MemoryToken;
use osciemu::emulator::Emulator;

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg IMAGE: -i --image +required +takes_value "Image to load into memory")
            (@arg BIOS: -b --bios +required +takes_value "BIOS to load")
            (@arg STEP: --step "Walk through in stepping mode")
        )
        .get_matches();

    let image_filename = matches.value_of("IMAGE").unwrap();
    let bios_filename = matches.value_of("BIOS").unwrap();
    let step_mode = matches.is_present("STEP");
    let mut image_vec: Vec<u8> = Vec::new();
    let mut bios_vec: Vec<u8> = Vec::new();

    let _ = File::open(image_filename)
        .expect("Can’t open image file")
        .read_to_end(&mut image_vec);
    let _ = File::open(bios_filename)
        .expect("Can’t open bios file")
        .read_to_end(&mut bios_vec);

    let mut emulator = Emulator::new();
    {
        let img_mem = MemoryToken::new(SliceMemory(image_vec.into_boxed_slice()));
        let bios_mem = MemoryToken::new(SliceMemory(bios_vec.into_boxed_slice()));
        emulator.memory.mount(0, &img_mem);
        emulator.memory.mount(0x80000000, &bios_mem);
    }

    for count in 1.. {
        if emulator.memory.get(addresses::FLAGS_START_ADDRESS) & (1 << addresses::FLAG0_HALT) != 0 {
            break;
        }
        println!("count: {:4}, ip: 0x{:08X}, r0: 0x{:08X}, r1: 0x{:08X}, r2: 0x{:08X}, r3: \
                  0x{:08X}",
                 count,
                 emulator.ip,
                 emulator.memory.get(addresses::REGISTERS_START_ADDRESS + 0),
                 emulator.memory.get(addresses::REGISTERS_START_ADDRESS + 4),
                 emulator.memory.get(addresses::REGISTERS_START_ADDRESS + 8),
                 emulator.memory.get(addresses::REGISTERS_START_ADDRESS + 12));
        emulator.step();
        if step_mode {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);
        }
    }
}
