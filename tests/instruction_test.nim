include ../osci/instruction
from ../osci/memory import newArrayMemory
import unittest

suite "instruction":
  test "idempotence":
    var
      a, b: Instruction = Instruction()
      am = newArrayMemory(16)
    a.op_a = 0x00010203
    a.op_b = 0x04050607
    a.target = 0x08090a0b
    a.jmp = 0x0c0d0e0f
    a.serialize(am, 0'u32)
    b.deserialize(am, 0'u32)
    check(a.op_a == b.op_a)
    check(a.op_b == b.op_b)
    check(a.target == b.target)
    check(a.jmp == b.jmp)
