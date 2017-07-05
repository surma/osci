type ZeroMemory* = object of Memory
  ##``ZeroMemory`` is a memory that always reads zero and discards writes.
  size: int

proc newZeroMemory*(size: int): ZeroMemory =
  ## Creates a new ``ZeroMemory`` with given size.
  ZeroMemory(size: size)

method size(zm: var ZeroMemory): int =
  zm.size

method get(zm: var ZeroMemory, address: uint32): uint32 =
  0

method set(zm: var ZeroMemory, address: uint32, value: uint32) =
  discard
