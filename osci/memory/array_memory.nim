type ArrayMemory* = object of Memory
  ##[
    ``ArrayMemory`` is a straight-up memory implementation backed by a chunk of memory (a
    ``seq[uint8]``).
  ]##
  data: seq[uint8]

proc newArrayMemory*(size: int): ArrayMemory =
  ## Creates a new ``ArrayMemory`` with an empty sequence of given size.
  ArrayMemory(data: newSeq[uint8](size))

proc newArrayMemory*(data: seq[uint8]): ArrayMemory =
  ## Creates a new ``ArrayMemory`` with the given sequence as the initial value.
  ArrayMemory(data: data)

method size(am: var ArrayMemory): int =
  am.data.len

method get(am: var ArrayMemory, address: uint32): uint32 =
  return
    (uint32(am.data[int(address) + 3]) shl 24) or
    (uint32(am.data[int(address) + 2]) shl 16) or
    (uint32(am.data[int(address) + 1]) shl 08) or
    (uint32(am.data[int(address) + 0]) shl 00)

method set(am: var ArrayMemory, address: uint32, value: uint32) =
  am.data[int(address + 0)] = uint8(value shr 00)
  am.data[int(address + 1)] = uint8(value shr 08)
  am.data[int(address + 2)] = uint8(value shr 16)
  am.data[int(address + 3)] = uint8(value shr 24)
  discard
