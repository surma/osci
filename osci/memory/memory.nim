from math import nil

const WORD_SIZE*: int = 4
const MAX_ADDRESS*: int32 = 0x7FFFFFFF'i32
const MAX_SIZE*: int = int(MAX_ADDRESS) + 1
const BIOS_ADDRESS*: int32 = 0x40000000'i32

const FLAG_HALT*: int = 0
const FLAG_BIOS_DONE*: int = 1
const NUM_FLAGS: int = 2
const NUM_FLAG_BYTES*: int = int(math.ceil(NUM_FLAGS/8))
const NUM_FLAG_WORDS*: int = int(math.ceil(NUM_FLAG_BYTES / 4))
const FLAGS0_ADDRESS*: int32 = MAX_SIZE - int32(NUM_FLAG_WORDS * WORD_SIZE)

const NUM_IVTS*: int = 1
const IVT0_ADDRESS*: int32 = FLAGS0_ADDRESS - NUM_IVTS * WORD_SIZE

const NUM_REGISTERS*: int = 4
const REGISTER0_ADDRESS*: int32 = IVT0_ADDRESS - NUM_REGISTERS * WORD_SIZE

const STACK_POINTER_ADDRESS*: int32 = REGISTER0_ADDRESS - WORD_SIZE

type Memory* = ref object of RootObj
  ## The base object for all memory implementations. By enforcing this “interface”,
  ## different implementations can be composed.

method size*(self: Memory): int {.base.} =
  ## Returns the size of the memory in bytes.
  quit "override missing"

method set*(self: Memory, address: int32, value: uint8) {.base.} =
  ## Sets the value of the byte at ``address`` to ``value``.
  quit "override missing"

method get*(self: Memory, address: int32): uint8 {.base.} =
  ## Gets the value of the byte at ``address``.
  quit "override missing"

proc readUint32*(self: Memory, address: int32): uint32 =
  ## Reads a ``uint32`` from the given memory at the given address.
  return
    uint32(self.get(address + 0)) shl 00 or
    uint32(self.get(address + 1)) shl 08 or
    uint32(self.get(address + 2)) shl 16 or
    uint32(self.get(address + 3)) shl 24

proc readInt32*(self: Memory, address: int32): int32 =
  ## Reads a ``int32`` from the given memory at the given address.
  cast[int32](self.readUint32(address))

proc writeUint32*(self: Memory, address: int32, value: uint32) =
  ## Writes a ``uint32`` to the given memory at the given address.
  self.set(address + 0, uint8(value shr 00))
  self.set(address + 1, uint8(value shr 08))
  self.set(address + 2, uint8(value shr 16))
  self.set(address + 3, uint8(value shr 24))

proc writeInt32*(self: Memory, address: int32, value: int32) =
  ## Writes a ``int32`` from the given memory at the given address.
  self.writeUint32(address, cast[uint32](value))
