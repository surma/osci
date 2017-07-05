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

const LOL2* = 9
