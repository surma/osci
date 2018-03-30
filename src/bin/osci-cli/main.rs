#[macro_use]
extern crate clap;
extern crate osciemu;
extern crate byteorder;

use std::vec::Vec;
use std::fs::File;
use std::io;
use osciemu::memory::SliceMemory;
use osciemu::emulator::Emulator;

use byteorder::{NetworkEndian, ReadBytesExt};

fn read_file_into_slice(filename: &str) -> Result<Box<[i32]>, io::Error> {
    let mut file = File::open(filename)?;
    let file_meta = file.metadata()?;
    let size = file_meta.len() as usize;
    let mut vec = Vec::<i32>::with_capacity(size);
    vec.resize(size / 4, 0);
    let mut slice: Box<[i32]> = vec.into_boxed_slice();
    file.read_i32_into::<NetworkEndian>(slice.as_mut())?;
    Ok(slice)
}

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg IMAGE: -i --image +required +takes_value "Image to load into memory")
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

    let image_buf = read_file_into_slice(matches.value_of("IMAGE").unwrap())
                            .expect("Could not load image");
    let bios_buf = read_file_into_slice(matches.value_of("BIOS").unwrap())
                            .expect("Could not load bios");

    let step_mode = matches.is_present("STEP");


    let img_mem = Box::new(SliceMemory(image_buf));
    let bios_mem = Box::new(SliceMemory(bios_buf));
    let mut emulator = Emulator::new(img_mem, bios_mem);

    println!("FLAGS_START_ADDRESS = 0x{:08X}", osciemu::memory::address::FLAGS_START_ADDRESS);
    println!("IVT_START_ADDRESS = 0x{:08X}", osciemu::memory::address::IVT_START_ADDRESS);
    println!("REGISTERS_START_ADDRESS = 0x{:08X}", osciemu::memory::address::REGISTERS_START_ADDRESS);
    println!("STACK_POINTER_ADDRESS = 0x{:08X}", osciemu::memory::address::STACK_POINTER_ADDRESS);

    for count in RangeIterator::new(0, max_steps) {
        if emulator.is_halted() {
            break;
        }
        println!("count: {:4}, ip: 0x{:08X}, r0: 0x{:08X}, r1: 0x{:08X}, r2: 0x{:08X}, r3: \
                  0x{:08X}",
                 count,
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
