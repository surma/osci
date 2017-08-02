type ReadonlyMemory* = ref object of Memory
  ## A memory wrapper that discards all writes
  memory: Memory

proc newReadonlyMemory*(m: Memory): ReadonlyMemory =
  ## Creates a new ``ReadonlyMemory``, wrapping the given memory.
  ReadonlyMemory(memory: m)

method size*(rm: ReadonlyMemory): int =
  rm.memory.size

method get*(rm: ReadonlyMemory, address: int32): uint8 =
  rm.memory.get(address)

method set*(rm: ReadonlyMemory, address: int32, value: uint8) =
  discard
