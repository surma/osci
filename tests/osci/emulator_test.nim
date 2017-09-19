include ../../osci/emulator
import ../../osci/memory
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
      emu = newEmulator(
        mainMemory = newArrayMemory(@[4'i32, 5]),
        biosMemory = newArrayMemory(@[0'i32, 4, 0, 100]),
      )
    emu.step()
    check(emu.memory.readInt32(0) == -1)
    check(emu.ip == 100)

  test "halted behavior":
    var
      emu = newEmulator(
        mainMemory = newArrayMemory(@[4'i32, 5]),
        biosMemory = newArrayMemory(@[0'i32, 4, 0, 100]),
      )
    check(emu.ip == BIOS_ADDRESS)
    check(emu.memory.readInt32(0) == 4)
    emu.halted = true
    emu.step()
    check(emu.ip == BIOS_ADDRESS)
    check(emu.memory.readInt32(0) == 4)

  test "halted bit":
    var
      emu = newEmulator()

    check(emu.halted == false)
    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_HALT) and 1) == 0)

    emu.memory.set(FLAGS0_ADDRESS, 1 shl FLAG_HALT)
    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_HALT) and 1) == 1)
    check(emu.halted == true)

    emu.memory.set(FLAGS0_ADDRESS, 0 shl FLAG_HALT)
    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_HALT) and 1) == 0)
    check(emu.halted == false)

  test "biosDone behavior":
    var
      emu = newEmulator(
        biosMemory = newArrayMemory(@[0xFF'u8])
      )

    check(emu.memory.get(BIOS_ADDRESS) == 0xFF)
    emu.biosDone = true
    check(emu.memory.get(BIOS_ADDRESS) == 0x00)
    emu.biosDone = false
    check(emu.memory.get(BIOS_ADDRESS) == 0xFF)

  test "biosDone bit":
    var
      emu = newEmulator()

    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_BIOS_DONE) and 1) == 0)
    check(emu.biosDone == false)

    emu.memory.set(FLAGS0_ADDRESS, 1 shl FLAG_BIOS_DONE)
    check(emu.biosDone == true)
    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_BIOS_DONE) and 1) == 1)

    emu.memory.set(FLAGS0_ADDRESS, 0 shl FLAG_BIOS_DONE)
    check(emu.biosDone == false)
    check(((emu.memory.get(FLAGS0_ADDRESS) shr FLAG_BIOS_DONE) and 1) == 0)

  test "register":
    var
      emu = newEmulator(
        biosMemory = newArrayMemory(@[0'i32, 4, 0, 0]),
        mainMemory = newArrayMemory(@[8'i32, 3, 0, 0]),
      )
    emu.biosMemory.writeInt32(8, REGISTER0_ADDRESS)
    emu.step();
    check(emu.register(0) == 5)

  test "bios memory is readonly":
    var
      emu = newEmulator(
        biosMemory = newArrayMemory(@[0'i32, 4, 0, 0]),
        mainMemory = newArrayMemory(@[8'i32, 3, 0, 0]),
      )
    emu.biosMemory.writeInt32(8, BIOS_ADDRESS)
    emu.step();
    check(emu.memory.get(BIOS_ADDRESS) == 0)
