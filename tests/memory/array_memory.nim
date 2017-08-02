import unittest

suite "ArrayMemory":
  test "from byte sequence":
    var am = newArrayMemory(@[0'u8, 1, 2, 3, 4])
    for i in 0..<am.size:
      check(am.get(int32(i)) == uint8(i))

  test "from word sequence":
    var am = newArrayMemory(@[
      0x03020100'i32,
      0x07060504,
    ])
    for i in 0..<am.size:
      check(am.get(int32(i)) == uint8(i))

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
