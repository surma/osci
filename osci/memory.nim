## =================================
## Composable memory implementations
## =================================
##
## The ``memory`` module provides both the ``Memory`` object as well a couple of subtypes. These
## subtypes can be combined to build osci’s memory. By themselves, they are not necessarily
## compliant to osci’s specification.
##
## osci’s memory
## -------------
## osci’s memory is only addressable at a word boundary and only entire words can be read or
## written. A word is 4 bytes in little endian. osci always has 2^31 bytes of virtual memory. Not
## all the memory addresses are necessarily backed by physical memory. Reads from unbacked
## (“unmapped”) memory yield 0. Writes to unmapped memory are discarded.
##
## At boot, the BIOS memory of unspecified size is mapped to the address space at 2^30, shadowing
## the potentially existing physical memory at that address. The instruction pointer (IP) is set to
## 2^30. The BIOS memory can be unmapped by setting the ``bD`` flag. BIOS memory is read-only.
##
## The end of the address range is an area of mapped memory for control flags, peripherals and
## interrupts::
##
##   +---------------------------------------+ Address 0
##   |                 Word 0                |
##   |                 Word 1                |
##   |                   ...                 |
##   +---------------------------------------+ STACK_POINTER_ADDRESS
##   |              Stack Pointer            |
##   +---------------------------------------+ REGISTER0_ADDRESS
##   |                Register 0             |
##   |                   ...                 |
##   |                Register k             |
##   +---------------------------------------+ IVT0_ADDRESS
##   |               IVT Entry 0             |
##   |                   ...                 |
##   |               IVT Entry j             |
##   +---------------------------------------+ FLAGS0_ADDRESS
##   |              Flags Word 0             |
##   |                   ...                 |
##   |              Flags Word i             |
##   +---------------------------------------+ MAX_ADDRESS = 2^31-1
##
## Registers:
## - Register 0: General purpose
## - Register 1: General purpose
## - Register 2: General purpose
## - Register 3: General purpose
##
## IVTs:
## - IVT Entry 0: Unused
##
## Flags Word 0::
##
##   MSB                                   LSB
##   +---------------------------------------+
##   |    |    |    |    |    |    | bD | H  | Byte 0
##   +---------------------------------------+
##   |                 Unused                |
##   +---------------------------------------+
##   |                 Unused                |
##   +---------------------------------------+
##   |                 Unused                | Byte 3
##   +---------------------------------------+
##
## Flags:
##
## * ``biosDone`` (``bD``): Unmaps the BIOS from the address space
## * ``halt`` (``H``): Halts the CPU

include memory/memory
include memory/array_memory
include memory/null_memory
include memory/mapped_memory
include memory/hook_memory
include memory/readonly_memory
