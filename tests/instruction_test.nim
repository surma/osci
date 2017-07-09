include ../osci/instruction
from ../osci/memory import newArrayMemory
import unittest

suite "instruction":
  test "serialize/deserialize":
    var
      a, b: Instruction = newInstruction()
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

  test "execute - no jmp":
    var
      am = newArrayMemory(@[
        5'u8, 0'u8, 0'u8, 0'u8,
        4'u8, 0'u8, 0'u8, 0'u8,
        0'u8, 0'u8, 0'u8, 0'u8,
      ])
      ip: uint32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(0) == 5)
    check(am.readInt32(4) == 4)
    check(am.readInt32(8) == 1)
    check(ip == 16)

  test "execute - jmp":
    var
      am = newArrayMemory(@[
        4'u8, 0'u8, 0'u8, 0'u8,
        5'u8, 0'u8, 0'u8, 0'u8,
        0'u8, 0'u8, 0'u8, 0'u8,
      ])
      ip: uint32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(0) == 4)
    check(am.readInt32(4) == 5)
    check(am.readInt32(8) == -1)
    check(ip == 0x100)

  test "execute - negative numbers no jmp":
    var
      am = newArrayMemory(12)
      ip: uint32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    am.writeInt32(0, -4)
    am.writeInt32(4, -5)
    instr.execute(am, ip)
    check(am.readInt32(8) == 1)
    check(ip == 16)


  test "execute - negative numbers jmp":
    var
      am = newArrayMemory(12)
      ip: uint32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    am.writeInt32(0, -5)
    am.writeInt32(4, -4)
    instr.execute(am, ip)
    check(am.readInt32(8) == -1)
    check(ip == 0x100)
