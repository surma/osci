#[macro_use]
extern crate clap;
extern crate osciemu;

use std::io;
use std::path::Path;
use osciemu::utils::load_file;
use osciemu::loader;
use osciemu::memory::{Memory, SliceMemory};
use osciemu::emulator::Emulator;

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg MEMORY: -m --memory +takes_value "Memory image to load")
            (@arg BIOS: -b --bios +required +takes_value "BIOS image to load")
            (@arg STEP: --step "Walk through in stepping mode")
            (@arg MAX_STEP: --maxstep +takes_value "Maximum number of CPU cycles (0 means infinite)")
            (@arg PRINT: --print +takes_value "Addresses to print after CPU halts")
        ).get_matches();

    let max_steps = matches
        .value_of("MAX_STEP")
        .map(|s| s.parse::<usize>().expect("--max-step needs to be a number"))
        .unwrap_or(0);

    let prints = matches.value_of("PRINT").map(|s| -> Vec<u32> {
        s.split(",")
            .map(|s| u32::from_str_radix(s.trim(), 16).expect("Invalid address"))
            .collect()
    });

    let image_mem = matches
        .value_of("MEMORY")
        .ok_or(loader::LoadError::new())
        .map(|path| Path::new(path))
        .and_then(load_file)
        .unwrap_or_else(|_err| Box::new(SliceMemory::new(0)));

    let bios_mem = matches
        .value_of("BIOS")
        .ok_or(loader::LoadError::new())
        .map(|path| Path::new(path))
        .and_then(load_file)
        .expect("Could not load bios");

    let step_mode = matches.is_present("STEP");

    let mut emulator = Emulator::new(bios_mem, image_mem);

    for count in RangeIterator::new(1, max_steps) {
        if emulator.is_halted() {
            println!("cycles: {:4}  HALTED\n", count);
            break;
        }
        println!(
            "cycles: {:4}, ip: 0x{:08X}, r0: 0x{:08X}, r1: 0x{:08X}, r2: 0x{:08X}, r3: \
             0x{:08X}",
            count,
            emulator.ip,
            emulator.get_register(0),
            emulator.get_register(1),
            emulator.get_register(2),
            emulator.get_register(3)
        );
        emulator.step();
        if step_mode {
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
        }
    }
    if emulator.is_halted() && prints.is_some() {
        let result = prints
            .unwrap()
            .iter()
            .map(|addr| format!("0x{:08X}", emulator.memory.get(*addr as usize)))
            .collect::<Vec<String>>()
            .join(", ");
        println!("Final state:");
        println!("{}", result);
    }
    if !emulator.is_halted() {
        std::process::exit(1);
    }
}

struct RangeIterator {
    count: usize,
    max: usize,
}

impl RangeIterator {
    fn new(start: usize, max: usize) -> RangeIterator {
        RangeIterator { count: start, max }
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
