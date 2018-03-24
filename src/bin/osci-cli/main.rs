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
        )
        .get_matches();

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
