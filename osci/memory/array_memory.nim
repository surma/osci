type ArrayMemory* = ref object of Memory
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

method size*(am: ArrayMemory): int =
  am.data.len

method get*(am: ArrayMemory, address: uint32): uint8 =
  am.data[int(address)]

method set*(am: ArrayMemory, address: uint32, value: uint8) =
  am.data[int(address + 0)] = value
