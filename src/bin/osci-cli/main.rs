#[macro_use]
extern crate clap;
extern crate osciemu;

use std::io;
use osciemu::utils::{load_file};
use osciemu::memory::{Memory, SliceMemory};
use osciemu::emulator::Emulator;
use osciemu::loader::{hexloader, rawloader};

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg IMAGE: -i --image +takes_value "Image to load into memory")
            (@arg BIOS: -b --bios +required +takes_value "BIOS to load")
            (@arg STEP: --step "Walk through in stepping mode")
            (@arg MAX_STEP: --maxstep +takes_value "Maximum number of CPU cycles (0 means infinite)")
        )
        .get_matches();

    let max_steps = matches.value_of("MAX_STEP")
                        .map(|s| {
                            s.parse::<usize>()
                                .expect("--max-step needs to be a number")
                        })
                        .unwrap_or(0);

    let image_mem = matches.value_of("IMAGE")
                            .ok_or(io::Error::new(io::ErrorKind::Other, "Parameter not specified"))
                            .and_then(load_file)
                            .unwrap_or_else(|_err| Box::new(SliceMemory::new(0)));

    let bios_mem = matches.value_of("BIOS")
                            .ok_or(io::Error::new(io::ErrorKind::Other, "Parameter not specified"))
                            .and_then(load_file)
                            .expect("Could not load bios");

    let step_mode = matches.is_present("STEP");

    let mut emulator = Emulator::new(image_mem, bios_mem);

    println!("FLAGS_START_ADDRESS = 0x{:08X}", osciemu::memory::address::FLAGS_START_ADDRESS);
    println!("IVT_START_ADDRESS = 0x{:08X}", osciemu::memory::address::IVT_START_ADDRESS);
    println!("REGISTERS_START_ADDRESS = 0x{:08X}", osciemu::memory::address::REGISTERS_START_ADDRESS);
    println!("STACK_POINTER_ADDRESS = 0x{:08X}", osciemu::memory::address::STACK_POINTER_ADDRESS);

    for _count in RangeIterator::new(0, max_steps) {
        if emulator.is_halted() {
            break;
        }
        println!("ip: 0x{:08X}, r0: 0x{:08X}, r1: 0x{:08X}, r2: 0x{:08X}, r3: \
                  0x{:08X}",
                 emulator.ip,
                 emulator.get_register(0),
                 emulator.get_register(1),
                 emulator.get_register(2),
                 emulator.get_register(3));
        emulator.step();
        if step_mode {
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
        }
    }
    if !emulator.is_halted() {
        std::process::exit(1);
    }
}

struct RangeIterator {
    count: usize,
    max: usize
}

impl RangeIterator {
    fn new(start: usize, max: usize) -> RangeIterator {
        RangeIterator {
            count: start,
            max,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.count == self.max && self.max != 0 {
            return None;
        }
        let result = Some(self.count);
        if self.count != std::usize::MAX {
            self.count = self.count + 1;
        }
        result
    }
}
