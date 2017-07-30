import unittest
import options
from future import `=>`

suite "HookMemory":
  test "size":
    var pm = newHookMemory()
    check(pm.size == 0)
    pm.size = () => 4
    check(pm.size == 4)

  test "get":
    var pm = newHookMemory()
    check(pm.get(0) == 0)
    check(pm.get(123) == 0)
    pm.get = (address: uint32) => uint8(address) + 1
    check(pm.get(0) == 1)
    check(pm.get(123) == 124)

  test "set":
    var
      pm = newHookMemory()
      called = false
    pm.set = proc (address: uint32, value: uint8) =
      called = true
    pm.set(0, 0)
    check(called == true)
