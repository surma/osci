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
    a.serialize(am, 0'i32)
    b.deserialize(am, 0'i32)
    check(a.op_a == b.op_a)
    check(a.op_b == b.op_b)
    check(a.target == b.target)
    check(a.jmp == b.jmp)

  test "fromMemory":
    var
      am = newArrayMemory(@[1'i32, 2, 3, 4])
      inst = fromMemory(am, 0)
    check(inst.op_a == 1)
    check(inst.op_b == 2)
    check(inst.target == 3)
    check(inst.jmp == 4)

  test "equality":
    var
      inst_a = newInstruction(1, 2, 3, 4)
      inst_b = newInstruction(1, 2, 3, 4)
    check(inst_a == inst_b)

  test "execute - no jmp":
    var
      am = newArrayMemory(@[5'i32, 4, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(0) == 5)
    check(am.readInt32(4) == 4)
    check(am.readInt32(8) == 1)
    check(ip == 16)

  test "execute - jmp on zero":
    var
      am = newArrayMemory(@[4'i32, 4, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(0) == 4)
    check(am.readInt32(4) == 4)
    check(am.readInt32(8) == 0)
    check(ip == 0x100)

  test "execute - jmp on negative result":
    var
      am = newArrayMemory(@[4'i32, 5, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(0) == 4)
    check(am.readInt32(4) == 5)
    check(am.readInt32(8) == -1)
    check(ip == 0x100)

  test "execute - negative numbers no jmp":
    var
      am = newArrayMemory(@[-4'i32, -5, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(8) == 1)
    check(ip == 16)


  test "execute - negative numbers jmp":
    var
      am = newArrayMemory(@[-5'i32, -4, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(8) == -1)
    check(ip == 0x100)

  test "execute - indirect ops no jmp":
    var
      am = newArrayMemory(@[0'i32, 12, 16, 5, 4, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(-4, -8, 20, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(20) == 1)
    check(ip == 16)

  test "execute - indirect ops jmp":
    var
      am = newArrayMemory(@[0'i32, 12, 16, 4, 5, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(-4, -8, 20, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(20) == -1)
    check(ip == 0x100)

  test "execute - indirect ops negative numbers no jmp":
    var
      am = newArrayMemory(@[0'i32, 12, 16, -4, -5, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(-4, -8, 20, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(20) == 1)
    check(ip == 16)

  test "execute - indirect ops negative numbers jmp":
    var
      am = newArrayMemory(@[0'i32, 12, 16, -5, -4, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(-4, -8, 20, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(20) == -1)
    check(ip == 0x100)

  test "execute - indirect target":
    var
      am = newArrayMemory(@[5'i32, 4, 12, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, -8, 0x100)
    instr.execute(am, ip)
    check(am.readInt32(12) == 1)

  test "execute - indirect jmp":
    var
      am = newArrayMemory(@[4'i32, 5, 0, 0x100])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, -12)
    instr.execute(am, ip)
    check(am.readInt32(8) == -1)
    check(ip == 0x100)

  test "execute - non-word boundary":
    var
      am = newArrayMemory(@[4'i32, 5, 0])
      ip: int32 = 0
      instr: Instruction = newInstruction(0, 4, 8, 31)
    instr.execute(am, ip)
    check(ip == 32)
