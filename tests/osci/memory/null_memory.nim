import unittest

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
