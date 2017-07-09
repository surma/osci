from memory import Memory, writeUint32, readUint32

##[
=================
CPU instructions
=================

An instruction consists of 4 words á 4 bytes. Each instruction is a set of 4 addresses ``op_a``,
``op_b``, ``target`` and ``jmp``. The execution of an instruction is equivalent to

::
  *target := *op_a - *op_b
  if (*target <= 0)
    GOTO jmp;

``jmp`` must be a multiple of the word size. If it’s not, it will be rounded to the next biggest
multiple of the word size.

osci is a 32-bit little endian CPU and instructions must be serialized accordingly.
]##

type
  InstructionObj* = object of RootObj
    op_a: uint32
    op_b: uint32
    target: uint32
    jmp: uint32
  Instruction* = ref InstructionObj

proc serialize*(instr: Instruction, m: Memory, address: uint32) =
  m.writeUint32(address + 00, instr.op_a)
  m.writeUint32(address + 04, instr.op_b)
  m.writeUint32(address + 08, instr.target)
  m.writeUint32(address + 12, instr.jmp)

proc deserialize*(instr: Instruction, m: Memory, address: uint32) =
  instr.op_a = m.readUint32(address + 00)
  instr.op_b = m.readUint32(address + 04)
  instr.target = m.readUint32(address + 08)
  instr.jmp = m.readUint32(address + 12)
