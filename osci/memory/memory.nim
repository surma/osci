type Memory* = object of RootObj
  ##[
    ``Memory`` is the base object for all memory implementations. By enforcing this “interface”,
    different implementations can be composed.
  ]##

method size(m: var Memory): int {.base.} =
  ## ``size`` returns the size of the memory in bytes.
  quit "override missing"

method set(m: var Memory, address: uint32, value: uint32) {.base.} =
  ## ``set`` sets the value of the word starting at ``address`` to ``value``.
  quit "override missing"

method get(m: var Memory, address: uint32): uint32 {.base.} =
  ## ``get`` gets the value of the word starting at ``address``.
  quit "override missing"
