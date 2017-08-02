type NullMemory* = ref object of Memory
  ## A memory implementation that always reads zero and discards writes.

proc newNullMemory*(): NullMemory =
  ## Creates a new ``NullMemory`.
  NullMemory()

method size*(nm: NullMemory): int =
  MAX_SIZE

method get*(nm: NullMemory, address: int32): uint8 =
  0

method set*(nm: NullMemory, address: int32, value: uint8) =
  discard
