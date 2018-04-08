//! Bindings for wasm.

use std::io::Cursor;

use loader::hexloader;
use emulator::Emulator;
use memory::Memory;

extern "C" {
    fn _js_print(addr: usize, size: usize);
}

#[cfg(target_arch = "wasm32")]
pub fn js_print(s: &str) {
    unsafe {
        _js_print(s.as_ptr() as usize, s.len());
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn do_it() -> i32 {
    let mut bios_code = Cursor::new(
        "
        ## Calculate 0x10 - 0x3 and store it in register 0
        40000008 40000009 7FFFFFF9 0
        0 0 0 40000000
        10 3
        ",
    );
    let bios = hexloader::load(&mut bios_code).unwrap();
    let mut emu = Emulator::from_bios_only(bios);
    for _ in 0..2 {
        for _ in 0..1_000_000 {
            emu.step();
        }
    }
    emu.get_register(0)
}
