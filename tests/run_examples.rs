extern crate osciemu;

use std::{fmt, io, fs, os, error, num};
use std::error::Error;
use std::io::{BufRead};
use std::path::{Path, PathBuf};

use osciemu::emulator::Emulator;
use osciemu::memory::{Memory, SliceMemory};
use osciemu::utils;

#[test]
fn run_examples() {
    let examples = fs::read_dir("./examples")
                      .unwrap()
                      .map(|entry| entry.unwrap().path());
    for example_path in examples {
        println!("Running {:?}...", example_path);
        run_example(&example_path);
    }
}

fn run_example(path: &Path) {
    let files: Vec<PathBuf> = fs::read_dir(path)
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .collect();

    let expect_file = files.iter()
                           .find(|file| file.file_name().unwrap().to_str().unwrap().contains(".expect."))
                           .expect(&format!("No expect file found for {:?}", path));

    let expect = parse_expect(&expect_file)
                    .expect(&format!("Could not parse expect file for {:?}", path));

    let bios = files.iter()
                    .find(|path| path.as_os_str().to_str().unwrap().contains(".bios."))
                    .map(|path| utils::load_file(path).unwrap())
                    .unwrap();

    let memory = files.iter()
                    .find(|path| path.as_os_str().to_str().unwrap().contains(".memory."))
                    .map(|path| utils::load_file(path).unwrap())
                    .unwrap_or_else(|| Box::new(SliceMemory::new(0)));

    let mut emu = Emulator::new(memory, bios);

    let max = 100;
    for i in 1..max {
        if emu.is_halted() {
            break;
        }
        emu.step();
        if i == max - 1 {
            panic!("Test never halted");
        }

    }

    for (addr, value) in expect {
        assert_eq!(emu.memory.get(addr), value);
    }
}

#[derive(Debug)]
enum ExpectError {
    IoErr(io::Error),
    ParseIntErr(num::ParseIntError)
}

impl fmt::Display for ExpectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl error::Error for ExpectError {
    fn description(&self) -> &str {
        match self {
            &ExpectError::IoErr(ref err) => err.description(),
            &ExpectError::ParseIntErr(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for ExpectError {
    fn from(err: io::Error) -> Self {
        ExpectError::IoErr(err)
    }
}

impl From<num::ParseIntError> for ExpectError {
    fn from(err: num::ParseIntError) -> Self {
        ExpectError::ParseIntErr(err)
    }
}

fn parse_expect(file: &PathBuf) -> Result<Vec<(usize, i32)>, ExpectError> {
    let mut v = Vec::<(usize, i32)>::new();
    for line in io::BufReader::new(fs::File::open(file)?).lines() {
        // TODO: Do this without a copy
        let items: Vec<String> = line?.split("=").map(|item| String::from(item.trim())).collect();
        let addr = usize::from_str_radix(items.get(0).unwrap(), 16)?;
        let value = i32::from_str_radix(items.get(1).unwrap(), 16)?;
        v.push((addr, value))
    }
    Ok(v)
}
