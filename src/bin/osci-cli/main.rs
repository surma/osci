#[macro_use]
extern crate clap;
extern crate osciemu;

use std::vec::Vec;
use std::fs::File;
use std::io;
use std::io::Read;
use osciemu::memory::SliceMemory;
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

    let img_mem = SliceMemory(image_vec.into_boxed_slice());
    let bios_mem = SliceMemory(bios_vec.into_boxed_slice());
    let mut emulator = Emulator::new(img_mem, bios_mem);

    for count in 1.. {
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
}
