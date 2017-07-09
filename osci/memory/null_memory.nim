type NullMemory* = ref object of Memory
  ##``NullMemory`` is a memory that always reads zero and discards writes.

proc newNullMemory*(): NullMemory =
  ## Creates a new ``NullMemory`` with given size.
  NullMemory()

method size*(nm: NullMemory): int =
  MAX_SIZE

method get*(nm: NullMemory, address: uint32): uint8 =
  0

method set*(nm: NullMemory, address: uint32, value: uint8) =
  discard
