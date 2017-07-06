include ../osci/memory
import unittest
from options import isNone, get

suite "ArrayMemory":
  test "size":
    var am = newArrayMemory(128)
    check(am.size == 128)

  test "get":
    let data: seq[uint8] = @[0'u8, 1'u8, 2'u8, 3'u8]
    var am = newArrayMemory(data)
    check(am.get(0) == 0x03020100)

  test "set":
    let data: seq[uint8] = newSeq[uint8](9)
    var am = newArrayMemory(data)
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
  proc numItems(dll: DoublyLinkedList[Mount]): int =
    var
      i = 0
      node = dll.head
    while node != nil:
      inc i
      node = node.next
    return i

  test "size":
    var mm = newMappedMemory()
    check(mm.size == 0xFFFFFFFF)

  test "mount":
    var mm = newMappedMemory()
    check(numItems(mm.mounts) == 0)
    var nm = newNullMemory(1)
    mm.mount(nm, 0)
    check(numItems(mm.mounts) == 1)

  test "memoryAtAddress":
    var mm = newMappedMemory()
    check(mm.memoryAtAddress(0).isNone())
    var nm = newNullMemory(16)
    mm.mount(nm, 0)
    check(mm.memoryAtAddress(0).get().memory == nm)
    check(mm.memoryAtAddress(8).get().memory == nm)
    check(mm.memoryAtAddress(15).get().memory == nm)
    check(mm.memoryAtAddress(16).isNone())
