#[macro_use]
extern crate clap;
extern crate osciemu;

use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use osciemu::memory::{SliceMemory, MappedMemory};
use osciemu::instruction::Instruction;

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg IMAGE: -i --image +required +takes_value "Image to load into memory")
            (@arg BIOS: -b --bios +required +takes_value "BIOS to load")
        )
        .get_matches();

    let image_filename = matches.value_of("IMAGE").unwrap();
    let bios_filename = matches.value_of("BIOS").unwrap();
    let mut image_vec: Vec<u8> = Vec::new();
    let mut bios_vec: Vec<u8> = Vec::new();

    File::open(image_filename)
        .expect("Can’t open image file")
        .read_to_end(&mut image_vec);
    File::open(bios_filename)
        .expect("Can’t open bios file")
        .read_to_end(&mut bios_vec);

    let mut memory = MappedMemory::new();
    let image = memory.mount(0, SliceMemory(image_vec.into_boxed_slice()));
    let bios = memory.mount(0x80000000, SliceMemory(bios_vec.into_boxed_slice()));
    let mut ip = 0x80000000;

    // loop {
    //     let instr = Instruction::from_memory(ip, memory);
    //     instr.execute(&mut ip, &mut memory);
    // }

}
