import unittest

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
