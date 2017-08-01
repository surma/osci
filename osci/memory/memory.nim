from math import nil

const WORD_SIZE: int = 4
const MAX_ADDRESS*: uint32 = 0xFFFFFFFF'u32
const MAX_SIZE*: int = int(MAX_ADDRESS) + 1
const BIOS_ADDRESS*: uint32 = 0x80000000'u32
const NUM_REGISTERS*: int = 4
const REGISTER0_ADDRESS*: uint32 = MAX_SIZE - NUM_REGISTERS * WORD_SIZE
const IVT_RETURN_ADDRESS*: uint32 = REGISTER0_ADDRESS - 1 * WORD_SIZE
const NUM_IVTS*: int = 0
const IVT0_ADDRESS*: uint32 = IVT_RETURN_ADDRESS - NUM_IVTS * WORD_SIZE
const NUM_FLAGS: int = 2
const NUM_FLAG_BYTES*: int = int(math.ceil(NUM_FLAGS/8))
const NUM_FLAG_WORDS*: int = int(math.ceil(NUM_FLAG_BYTES / 4))
const FLAGS0_ADDRESS*: uint32 = IVT0_ADDRESS - uint32(NUM_FLAG_WORDS)
const FLAG_HALT*: int = 0
const FLAG_BIOS_DONE*: int = 1

type Memory* = ref object of RootObj
  ## The base object for all memory implementations. By enforcing this “interface”,
  ## different implementations can be composed.

method size*(m: Memory): int {.base.} =
  ## Returns the size of the memory in bytes.
  quit "override missing"

method set*(m: Memory, address: uint32, value: uint8) {.base.} =
  ## Sets the value of the byte at ``address`` to ``value``.
  quit "override missing"

method get*(m: Memory, address: uint32): uint8 {.base.} =
  ## Gets the value of the byte at ``address``.
  quit "override missing"

proc readUint32*(m: Memory, address: uint32): uint32 =
  ## Reads a ``uint32`` from the given memory at the given address.
  return
    uint32(m.get(address + 0)) shl 00 or
    uint32(m.get(address + 1)) shl 08 or
    uint32(m.get(address + 2)) shl 16 or
    uint32(m.get(address + 3)) shl 24

proc readInt32*(m: Memory, address: uint32): int32 =
  ## Reads a ``int32`` from the given memory at the given address.
  cast[int32](m.readUint32(address))

proc writeUint32*(m: Memory, address: uint32, value: uint32) =
  ## Writes a ``uint32`` to the given memory at the given address.
  m.set(address + 0, uint8(value shr 00))
  m.set(address + 1, uint8(value shr 08))
  m.set(address + 2, uint8(value shr 16))
  m.set(address + 3, uint8(value shr 24))

proc writeInt32*(m: Memory, address: uint32, value: int32) =
  ## Writes a ``int32`` from the given memory at the given address.
  m.writeUint32(address, cast[uint32](value))
