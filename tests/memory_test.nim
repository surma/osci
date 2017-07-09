include ../osci/memory
import unittest
from options import isNone, get

suite "Memory":
  test "readUint32":
    var am = newArrayMemory(@[0'u8, 1'u8, 2'u8, 3'u8])
    check(am.readUint32(0) == 0x03020100)

  test "readInt32":
    var am = newArrayMemory(@[0xFF'u8, 0xFF'u8, 0xFF'u8, 0xFF'u8])
    check(am.readInt32(0) == -1)

  test "writeUint32":
    var am = newArrayMemory(4)
    am.writeUint32(0, 0x03020100)
    check(am.readUint32(0) == 0x03020100)

  test "writeInt32":
    var am = newArrayMemory(4)
    am.writeInt32(0, -1)
    check(am.readUint32(0) == 0xFFFFFFFF'u32)

suite "ArrayMemory":
  test "size":
    var am = newArrayMemory(128)
    check(am.size == 128)

  test "get":
    var am = newArrayMemory(@[0'u8, 1'u8, 2'u8, 3'u8])
    check(am.get(0) == 0)

  test "set":
    var am = newArrayMemory(newSeq[uint8](9))
    check(am.get(0) == 0)
    am.set(0, 4)
    check(am.get(0) == 4)

suite "NullMemory":
  test "size":
    var nm = newNullMemory()
    check(nm.size == MAX_SIZE)

  test "get":
    var nm = newNullMemory()
    check(nm.get(0) == 0)

  test "set":
    var nm = newNullMemory()
    check(nm.get(0) == 0)
    nm.set(0, 4)
    check(nm.get(0) == 0)

suite "MappedMemory":
  test "size":
    var mm = newMappedMemory()
    check(mm.size == 0xFFFFFFFF)

  test "numMounts":
    var mm = newMappedMemory()
    check(mm.numMounts == 0)
    mm.mount(newNullMemory(), 0)
    check(mm.numMounts == 1)
    mm.mount(newNullMemory(), 1)
    check(mm.numMounts == 2)

  test "memoryAtAddress":
    var mm = newMappedMemory()
    check(mm.memoryAtAddress(0).isNone())
    var am = newArrayMemory(16)
    mm.mount(am, 0)
    check(mm.memoryAtAddress(0).get().memory == am)
    check(mm.memoryAtAddress(8).get().memory == am)
    check(mm.memoryAtAddress(15).get().memory == am)
    check(mm.memoryAtAddress(16).isNone())

  test "get":
    var mm = newMappedMemory()
    mm.mount(newNullMemory(), 0)
    mm.mount(newArrayMemory(@[1'u8]), 1)
    mm.mount(newArrayMemory(@[2'u8]), 3)
    check(mm.get(0) == 0)
    check(mm.get(1) == 1)
    check(mm.get(2) == 0)
    check(mm.get(3) == 2)

  test "set":
    var
      mm = newMappedMemory()
      am1 = newArrayMemory(@[1'u8])
      am2 = newArrayMemory(@[2'u8])
    mm.mount(newNullMemory(), 0)
    mm.mount(am1, 1)
    mm.mount(am2, 2)
    mm.set(1, 0x11)
    mm.set(2, 0x12)
    check(am1.get(0) == 0x11)
    check(am2.get(0) == 0x12)

