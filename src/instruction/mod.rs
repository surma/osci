use memory::Memory;

/// Data object for a single instruction.
///
/// An instruction consists of 4 words á 4 bytes. Each instruction can be
/// intepreted as 4 address `[op_a, op_b, target, jmp]`. the
/// execution of an instruction is equivalent to
///
/// ```text
///   *target := *op_a - *op_b
///   if (*target <= 0)
///     GOTO jmp;
/// ```
///
/// `jmp` must be a multiple of the word size. If it’s not, it will be rounded
/// to the next biggest multiple of the word size.
///
/// osci is a 32-bit little endian CPU and instructions must be serialized
/// accordingly.
///
/// # Examples
/// ```
/// use osciemu::memory::SliceMemory;
/// use osciemu::instruction::Instruction;
///
/// let mut ip = 0;
/// let mut m = SliceMemory::from_slice_u32(16, &[
///     0, 4, 8, 128,
/// ]);
/// Instruction::execute_at(&mut ip, &mut m);
/// assert_eq!(ip, 128);
/// ```
pub struct Instruction {
    /// Address of operand A
    pub op_a: u32,
    /// Address of operand B
    pub op_b: u32,
    /// Address to store result of `*A - *B`
    pub target: u32,
    /// Address to jump to when `*target <= 0`
    pub jmp: u32,
}

impl Instruction {
    /// Deserializes an instruction from memory at the given address.
    pub fn from_memory(addr: usize, mem: &Memory) -> Instruction {
        Instruction {
            op_a: mem.get(addr),
            op_b: mem.get(addr + 4),
            target: mem.get(addr + 8),
            jmp: mem.get(addr + 12),
        }
    }

    /// Serializes the instruction to memory at the given adress.
    pub fn serialize(&self, addr: usize, mem: &mut Memory) {
        mem.set(addr + 0, self.op_a);
        mem.set(addr + 4, self.op_b);
        mem.set(addr + 8, self.target);
        mem.set(addr + 12, self.jmp);
    }

    /// Executes the instruction using `mem` for reads and writes and
    /// adjusting `ip` appropriately.
    pub fn execute(&self, ip: &mut usize, mem: &mut Memory) {
        let a = mem.get(self.op_a as usize) as i32;
        let b = mem.get(self.op_b as usize) as i32;
        let r = a - b;
        mem.set(self.target as usize, r as u32);
        *ip = if r <= 0 { self.jmp as usize } else { *ip + 16 }
    }

    /// Executes the instruction in memory at the given address, adjusting the
    // `ip` appropriately.
    pub fn execute_at(ip: &mut usize, mem: &mut Memory) {
        let instr = Instruction::from_memory(*ip, mem);
        instr.execute(ip, mem);
    }
}

#[cfg(test)]
mod tests {
    use memory::{Memory, SliceMemory};

    #[test]
    fn execute() {
        let mut ip = 0;
        let mut m = SliceMemory::from_slice_u32(16, &[1, 2, 0, 0]);
        let i1 = super::Instruction {
            op_a: 0,
            op_b: 4,
            target: 8,
            jmp: 128,
        };
        i1.execute(&mut ip, &mut m);
        assert_eq!(m.get(8) as i32, -1);
        assert_eq!(ip, 128);

        ip = 0;
        let i2 = super::Instruction {
            op_a: 4,
            op_b: 0,
            target: 8,
            jmp: 128,
        };
        i2.execute(&mut ip, &mut m);
        assert_eq!(m.get(8), 1);
        assert_eq!(ip, 16);
    }

    #[test]
    fn execute_at() {
        let mut ip = 0;
        let mut m = SliceMemory::from_slice_u32(64,
                                                &[48, 52, 56, 60, 48, 48, 56, 60, 0, 0, 0, 0, 2,
                                                  1, 0, 0]);
        super::Instruction::execute_at(&mut ip, &mut m);
        assert_eq!(m.get(56), 1);
        assert_eq!(ip, 16);
        super::Instruction::execute_at(&mut ip, &mut m);
        assert_eq!(m.get(56), 0);
        assert_eq!(ip, 60);

    }
}
