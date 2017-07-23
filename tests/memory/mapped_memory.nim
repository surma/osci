import unittest
from options import isNone, get

suite "MappedMemory":
  test "size":
    var mm = newMappedMemory()
    check(mm.size == MAX_SIZE)

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

  test "remount":
    var mm = newMappedMemory()
    mm.mount(newNullMemory(), 0)
    var oldM = newArrayMemory(@[1'u8, 1'u8])
    mm.mount(oldM, 1)
    check(mm.get(1) == 1)
    check(mm.get(2) == 1)
    mm.remount(oldM, newArrayMemory(@[2'u8]))
    check(mm.isMounted(oldM) == false)
    check(mm.get(1) == 2)
    check(mm.get(2) == 0)

  test "isMounted":
    var
      mm = newMappedMemory()
      am1 = newArrayMemory(16)
      am2 = newArrayMemory(16)

    check(not mm.isMounted(am1))
    check(not mm.isMounted(am2))
    mm.mount(am1, 0)
    check(mm.isMounted(am1))
    check(not mm.isMounted(am2))
    mm.mount(am2, 0)
    check(mm.isMounted(am1))
    check(mm.isMounted(am2))

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
