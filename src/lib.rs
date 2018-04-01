//! Modules for an osci emulator.
//!
//! osci is a minimalistic one instruction set computer (OISC). It was originally created as an exercise with the goal to ultimately be implemented in hardware. osci is inspired by [SUBLEQ], but has been changed to simplify implementation and usage.
//!
//! - For more details on the instruction set, see the `instruction` module.
//! - For more details on the architecture and memory layout, see the `memory` module.
//!
//! [SUBLEQ]: https://esolangs.org/wiki/Subleq
pub mod memory;
pub mod instruction;
pub mod emulator;
pub mod loader;
pub mod utils;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
