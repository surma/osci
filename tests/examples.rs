extern crate osciemu;

use std::fs;
use std::path::{Path, PathBuf};

use osciemu::emulator::Emulator;
use osciemu::memory::SliceMemory;
use osciemu::utils;

#[test]
fn run_examples() {
    let examples = fs::read_dir("./examples")
                      .unwrap()
                      .map(|entry| entry.unwrap().path());
    for example_path in examples {
        run_example(&example_path);
    }
    assert_eq!(1, 2);
}

fn run_example(path: &Path) {
    let files: Vec<PathBuf> = fs::read_dir(path)
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .collect();

    let expect = match files.iter().find(|file| file.ends_with(".expect.txt")) {
        Some(file) => file,
        None => {
            println!("No expect file found for {:?}", path);
            return;
        }
    };
    println!("Expect file: {:?}", expect);

    let tests = files.iter()
                     .map(|path| utils::load_file(path.as_os_str().to_str().unwrap()))
                     .filter(|memory| memory.is_ok())
                     .map(|memory| memory.unwrap());

    for test in tests {
        let mut emu = Emulator::new(Box::from(SliceMemory::new(0)), test);
        for _i in 1..100000 {
            if emu.is_halted() {
                break;
            }
            emu.step();
        }
        println!("Test: {}", emu.get_register(0));
    }
    assert!(false);
}

