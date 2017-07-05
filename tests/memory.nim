include ../osci/memory
import unittest

suite "Memory":
  setup:
    discard

  teardown:
    discard

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
