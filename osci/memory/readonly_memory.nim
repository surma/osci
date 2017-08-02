type ReadonlyMemory* = ref object of Memory
  ## A memory wrapper that discards all writes
  memory: Memory

proc newReadonlyMemory*(m: Memory): ReadonlyMemory =
  ## Creates a new ``ReadonlyMemory``, wrapping the given memory.
  ReadonlyMemory(memory: m)

method size*(self: ReadonlyMemory): int =
  self.memory.size

method get*(self: ReadonlyMemory, address: int32): uint8 =
  self.memory.get(address)

method set*(self: ReadonlyMemory, address: int32, value: uint8) =
  discard
