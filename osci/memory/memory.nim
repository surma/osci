type Memory* = ref object of RootObj
  ##[
    ``Memory`` is the base object for all memory implementations. By enforcing this “interface”,
    different implementations can be composed.
  ]##

method size*(m: Memory): int {.base.} =
  ## ``size`` returns the size of the memory in bytes.
  quit "override missing"

method set*(m: Memory, address: uint32, value: uint8) {.base.} =
  ## ``set`` sets the value of the word starting at ``address`` to ``value``.
  quit "override missing"

method get*(m: Memory, address: uint32): uint8 {.base.} =
  ## ``get`` gets the value of the word starting at ``address``.
  quit "override missing"

proc readUint32*(m: Memory, address: uint32): uint32 =
  ## ``readUint32` reads a ``uint32`` from the giving memory at the given address.
  return
    uint32(m.get(address + 0)) shl 00 or
    uint32(m.get(address + 1)) shl 08 or
    uint32(m.get(address + 2)) shl 16 or
    uint32(m.get(address + 3)) shl 24

proc writeUint32*(m: Memory, address: uint32, value: uint32) =
  ## ``writeUint32` writes a ``uint32`` to the giving memory at the given address.
  m.set(address + 0, uint8(value shr 00))
  m.set(address + 1, uint8(value shr 08))
  m.set(address + 2, uint8(value shr 16))
  m.set(address + 3, uint8(value shr 24))
