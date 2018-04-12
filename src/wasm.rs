//! Bindings for wasm.

use std::io::Cursor;
use std::vec::Vec;
use std::{slice, str, mem};

use loader::hexloader;
// use emulator::Emulator;
use memory::Memory;

extern "C" {
    #[cfg(target_arch = "wasm32")]
    fn _js_print(addr: usize, size: usize);
}

#[cfg(target_arch = "wasm32")]
pub fn js_print(s: &str) {
    unsafe {
        _js_print(s.as_ptr() as usize, s.len());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn js_print(s: &str) {
    println!("{}", s);
}

#[no_mangle]
pub extern "C" fn wasm__allocate_u8_slice(size: usize, fill: u8) -> usize {
    let mut vec = Vec::<u8>::with_capacity(size);
    vec.resize(size, fill);
    let slice_ptr = Box::into_raw(vec.into_boxed_slice());
    mem::forget(slice_ptr);
    slice_ptr as *mut () as usize
}

#[no_mangle]
pub extern "C" fn wasm__get_u8_slice_data_ptr(ptr: usize, len: usize) -> usize {
    let slice: &[u8];
    unsafe {
        slice = slice::from_raw_parts(ptr as *const () as *const u8, len)
    }

    &slice[0] as *const u8 as *const () as usize
}

#[no_mangle]
pub extern "C" fn loader__hexloader__load(ptr: usize, len: usize) -> usize {
    let slice;
    unsafe {
        slice = slice::from_raw_parts(ptr as *const () as *const u8, len);
    }
    let string = str::from_utf8(slice);
    if string.is_err() {
        return 0;
    }
    let mut c = Cursor::new(string.unwrap());
    let memory = hexloader::load(&mut c);
    if memory.is_err() {
        return 0;
    }

    Box::into_raw(memory.unwrap()) as *mut Memory as *mut () as usize
}
