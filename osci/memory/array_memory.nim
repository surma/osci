from sequtils import toSeq

type ArrayMemory* = ref object of Memory
  ## A straight-up memory implementation backed by a chunk of memory (a ``seq[uint8]``).
  data: seq[uint8]

proc newArrayMemory*(size: int): ArrayMemory =
  ## Creates a new ``ArrayMemory`` with an empty sequence of given size.
  ArrayMemory(data: newSeq[uint8](size))

proc newArrayMemory*(data: openArray[uint8]): ArrayMemory =
  ## Creates a new ``ArrayMemory`` with the given sequence as the initial value.
  ArrayMemory(data: toSeq(data.items))

method size*(self: ArrayMemory): int =
  self.data.len

method get*(self: ArrayMemory, address: int32): uint8 =
  self.data[int(address)]

method set*(self: ArrayMemory, address: int32, value: uint8) =
  self.data[int(address + 0)] = value
