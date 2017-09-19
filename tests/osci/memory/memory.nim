import unittest

suite "Memory":
  test "readUint32":
    var am = newArrayMemory(@[0'u8, 1'u8, 2'u8, 3'u8])
    check(am.readUint32(0) == 0x03020100'u32)

  test "readInt32":
    var am = newArrayMemory(@[0xFF'u8, 0xFF'u8, 0xFF'u8, 0xFF'u8])
    check(am.readInt32(0) == -1)

  test "writeUint32":
    var am = newArrayMemory(4)
    am.writeUint32(0, 0x03020100)
    check(am.readUint32(0) == 0x03020100'u32)

  test "writeInt32":
    var am = newArrayMemory(4)
    am.writeInt32(0, -1)
    check(am.readUint32(0) == 0xFFFFFFFF'u32)
