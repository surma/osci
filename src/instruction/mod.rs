//! osci instruction set.
//!
//! osci only has a single instruction inspired by [SUBLEQ]. osci has 4 operands instead of 3 to separate data source and data sink as well as support for indirect addressing.
//!
//! Each operand is a *signed* word á 32 bit. The operands are a set of addresses called `op_a`, `op_b`, `target` and `jmp`. If an operand is negative, it is considered indirect. The execution of an instruction is described by the following pseudo-code:
//!
//! ```text
//! if(op_a < 0) op_a = *(-op_a)
//! if(op_b < 0) op_b = *(-op_b)
//! if(target < 0) target = *(-target)
//! if(jmp < 0) jmp = *(-jmp)
//!
//! *target := *op_a - *op_b
//! if (*target <= 0)
//!   GOTO jmp;
//! ```
//!
//! [SUBLEQ]: https://esolangs.org/wiki/Subleq

use memory::Memory;
use std::fmt;

// TODO: Consider a flag to switch between absolute and relative addressing.

/// Data structure for a single instruction.
///
/// # Examples
///
/// ```
/// use osciemu::memory::SliceMemory;
/// use osciemu::instruction::Instruction;
///
/// let mut ip = 0;
/// let mut m = SliceMemory::from_slice(Box::new([
///     0, 1, 2, 128,
/// ]));
/// Instruction::execute_at(&mut ip, &mut m);
/// assert_eq!(ip, 128);
/// ```
pub struct Instruction {
    /// Address of operand A
    pub op_a: i32,
    /// Address of operand B
    pub op_b: i32,
    /// Address to store the result
    pub target: i32,
    /// Address to jump to when result is non-positive
    pub jmp: i32,
}

impl Instruction {
    /// Deserializes an instruction from memory at the given address.
    pub fn from_memory(addr: usize, mem: &Memory) -> Instruction {
        Instruction {
            op_a: mem.get(addr),
            op_b: mem.get(addr + 1),
            target: mem.get(addr + 2),
            jmp: mem.get(addr + 3),
        }
    }

    /// Serializes the instruction to memory at the given adress.
    pub fn serialize(&self, addr: usize, mem: &mut Memory) {
        mem.set(addr + 0, self.op_a);
        mem.set(addr + 1, self.op_b);
        mem.set(addr + 2, self.target);
        mem.set(addr + 3, self.jmp);
    }

    /// Executes the instruction using `mem` for reads and writes and
    /// adjusting `ip` appropriately.
    pub fn execute(&self, ip: &mut usize, mem: &mut Memory) {
        let mut op_a = self.op_a;
        let mut op_b = self.op_b;
        let mut target = self.target;
        let mut jmp = self.jmp;

        if op_a < 0 {
            op_a = mem.get(-op_a as usize);
        }
        if op_b < 0 {
            op_b = mem.get(-op_b as usize);
        }
        if target < 0 {
            target = mem.get(-target as usize);
        }
        if jmp < 0 {
            jmp = mem.get(-jmp as usize);
        }

        let a = mem.get(op_a as usize);
        let b = mem.get(op_b as usize);
        let r = a - b;
        mem.set(target as usize, r);
        *ip = if r <= 0 { jmp as usize } else { *ip + 4 }
    }

    /// Executes the instruction in memory at the given address, adjusting the
    // `ip` appropriately.
    pub fn execute_at(ip: &mut usize, mem: &mut Memory) {
        let instr = Instruction::from_memory(*ip, mem);
        instr.execute(ip, mem);
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Instruction {{ op_a: 0x{:08X}, op_b: 0x{:08X}, target: 0x{:08X}, jmp: 0x{:08X} }}",
            self.op_a, self.op_b, self.target, self.jmp
        )
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory};

    #[test]
    fn execute() {
        let mut ip = 0;
        let mut m = SliceMemory::from_slice(Box::new([1, 2, 0, 0]));
        let i = super::Instruction {
            op_a: 0,
            op_b: 1,
            target: 2,
            jmp: 128,
        };
        i.execute(&mut ip, &mut m);
        assert_eq!(m.get(2) as i32, -1);
        assert_eq!(ip, 128);

        let mut m = SliceMemory::from_slice(Box::new([1, 2, 0, 0]));
        let mut ip = 0;
        let i = super::Instruction {
            op_a: 1,
            op_b: 0,
            target: 2,
            jmp: 128,
        };
        i.execute(&mut ip, &mut m);
        assert_eq!(m.get(2), 1);
        assert_eq!(ip, 4);

        let mut m = SliceMemory::from_slice(Box::new([1, 2, 0, 0, 1, 8]));
        let mut ip = 0;
        let i = super::Instruction {
            op_a: -2,
            op_b: -4,
            target: -1,
            jmp: -5,
        };
        i.execute(&mut ip, &mut m);
        assert_eq!(m.get(2), -1);
        assert_eq!(ip, 8);
    }

    #[test]
    fn execute_at() {
        let mut ip = 0;
        let mut m = SliceMemory::from_slice(Box::new([
            12, 13, 14, 15, 12, 12, 14, 15, 0, 0, 0, 0, 2, 1, 0, 0
        ]));
        super::Instruction::execute_at(&mut ip, &mut m);
        assert_eq!(m.get(14), 1);
        assert_eq!(ip, 4);
        super::Instruction::execute_at(&mut ip, &mut m);
        assert_eq!(m.get(14), 0);
        assert_eq!(ip, 15);
    }
}
