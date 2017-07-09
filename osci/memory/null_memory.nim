type NullMemory* = ref object of Memory
  ##``NullMemory`` is a memory that always reads zero and discards writes.
  size: int

proc newNullMemory*(size: int): NullMemory =
  ## Creates a new ``NullMemory`` with given size.
  NullMemory(size: size)

method size*(nm: NullMemory): int =
  nm.size

method get*(nm: NullMemory, address: uint32): uint8 =
  0

method set*(nm: NullMemory, address: uint32, value: uint8) =
  discard
