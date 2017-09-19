import unittest

suite "ReadonlyMemory":
  test "size":
    var rm = newReadonlyMemory(newArrayMemory(@[1'u8, 2, 3]))
    check(rm.size == 3)

  test "get":
    var rm = newReadonlyMemory(newArrayMemory(@[1'u8, 2]))
    check(rm.get(0) == 1)
    check(rm.get(1) == 2)

  test "set":
    var rm = newReadonlyMemory(newArrayMemory(@[1'u8, 2]))
    rm.set(0, 9)
    rm.set(1, 9)
    check(rm.get(0) == 1)
    check(rm.get(1) == 2)
