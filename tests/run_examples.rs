extern crate osciemu;

use std::{error, fmt, fs, io, num};
use std::error::Error;
use std::io::BufRead;
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

fn is_supported_format(path: &Path) -> bool {
    let ext = path.extension();
    if ext.is_none() {
        return false;
    }
    let ext = ext.and_then(|ext| ext.to_str());
    utils::SUPPORTED_FORMATS
        .iter()
        .find(|supported_format| Some(**supported_format) == ext)
        .is_some()
}

fn is_bios_file(path: &Path) -> bool {
    return path.as_os_str().to_str().unwrap().contains(".bios.");
}

fn is_expect_file(path: &Path) -> bool {
    return path.as_os_str().to_str().unwrap().contains(".expect.");
}

fn is_memory_file(path: &Path) -> bool {
    return path.as_os_str().to_str().unwrap().contains(".memory.");
}

fn run_example(path: &Path) {
    let files: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    let expect_file = files
        .iter()
        .find(|path| is_expect_file(path))
        .expect(&format!("No expect file found for {:?}", path));

    let expect =
        parse_expect(&expect_file).expect(&format!("Could not parse expect file for {:?}", path));

    let bios_file = files
        .iter()
        .filter(|path| is_supported_format(path))
        .find(|path| is_bios_file(path))
        .expect(&format!("No bios file found for {:?}", expect_file));
    let bios =
        utils::load_file(bios_file).expect(&format!("Could not load bios file {:?}", bios_file));

    let memory = files
        .iter()
        .find(|path| is_memory_file(path))
        .map(|path| utils::load_file(path).unwrap())
        .unwrap_or_else(|| Box::new(SliceMemory::new(0)));

    let mut emu = Emulator::new(bios, memory);

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
    ParseIntErr(num::ParseIntError),
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
        let items: Vec<String> = line?
            .split("=")
            .map(|item| String::from(item.trim()))
            .collect();
        let addr = usize::from_str_radix(items.get(0).unwrap(), 16)?;
        let value = i32::from_str_radix(items.get(1).unwrap(), 16)?;
        v.push((addr, value))
    }
    Ok(v)
}
