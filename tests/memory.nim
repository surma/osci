include ../osci/memory
import unittest
from options import isNone, get

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
    var nm = newNullMemory(128)
    check(nm.size == 128)

  test "get":
    var nm = newNullMemory(4)
    check(nm.get(0) == 0)

  test "set":
    var nm = newNullMemory(4)
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
    mm.mount(newNullMemory(1), 0)
    check(mm.numMounts == 1)
    mm.mount(newNullMemory(1), 1)
    check(mm.numMounts == 2)

  test "memoryAtAddress":
    var mm = newMappedMemory()
    check(mm.memoryAtAddress(0).isNone())
    var nm = newNullMemory(16)
    mm.mount(nm, 0)
    check(mm.memoryAtAddress(0).get().memory == nm)
    check(mm.memoryAtAddress(8).get().memory == nm)
    check(mm.memoryAtAddress(15).get().memory == nm)
    check(mm.memoryAtAddress(16).isNone())
