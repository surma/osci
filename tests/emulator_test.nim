include ../osci/emulator
import ../osci/memory
import unittest

suite "emulator":
  test "memory":
    var
      mainMem1 = newArrayMemory(@[1'u8, 1'u8, 1'u8, 1'u8, 1'u8, 1'u8, 1'u8, 1'u8])
      mainMem2 = newArrayMemory(@[2'u8, 2'u8, 2'u8, 2'u8])
      emu = newEmulator()
    emu.mainMemory = mainMem1
    check(emu.memory.get(0) == 1)
    check(emu.memory.get(3) == 1)
    check(emu.memory.get(4) == 1)
    check(emu.memory.get(7) == 1)
    check(emu.memory.get(8) == 0)
    emu.mainMemory = mainMem2
    check(emu.memory.get(0) == 2)
    check(emu.memory.get(3) == 2)
    check(emu.memory.get(4) == 0)
    check(emu.memory.get(7) == 0)
    check(emu.memory.get(8) == 0)

  test "step":
    var
      emu = newEmulator(mainMemory = newArrayMemory(@[
        16'u8, 0'u8, 0'u8, 0'u8,
        20'u8, 0'u8, 0'u8, 0'u8,
        0'u8, 0'u8, 0'u8, 0'u8,
        100'u8, 0'u8, 0'u8, 0'u8,
        4'u8, 0'u8, 0'u8, 0'u8,
        5'u8, 0'u8, 0'u8, 0'u8,
      ]))
    emu.step()
    check(emu.memory.readInt32(0) == -1)
    check(emu.ip == 100)

  test "halted behavior":
    var
      emu = newEmulator(mainMemory = newArrayMemory(@[
        16'u8, 0'u8, 0'u8, 0'u8,
        20'u8, 0'u8, 0'u8, 0'u8,
        0'u8, 0'u8, 0'u8, 0'u8,
        100'u8, 0'u8, 0'u8, 0'u8,
        4'u8, 0'u8, 0'u8, 0'u8,
        5'u8, 0'u8, 0'u8, 0'u8,
      ]))
    check(emu.ip == 0)
    check(emu.memory.readInt32(0) == 16)
    emu.halted = true
    emu.step()
    check(emu.ip == 0)
    check(emu.memory.readInt32(0) == 16)

  test "biosDone behavior":
    var
      emu = newEmulator(biosMemory = newArrayMemory(@[0xFF'u8]))

    check(emu.memory.get(BIOS_ADDRESS) == 0xFF)
    emu.biosDone = true
    check(emu.memory.get(BIOS_ADDRESS) == 0x00)
