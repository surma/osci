from lists import DoublyLinkedList,DoublyLinkedNode,nodes,append
from tables import Table
import options
from future import `->`, `=>`

type
  SetHook* = (a: int32, b: uint8) -> void
  GetHook* = (int32) -> uint8
  SizeHook* = () -> int
  HookMemory* = ref object of Memory
    ## Externalizes the interface as callbacks (or “hooks”). ``get`` and ``size`` will default to
    ## ``0`` if no hook has been set.
    Fget: Option[GetHook]
    Fset: Option[SetHook]
    Fsize: Option[SizeHook]

proc newHookMemory*(): HookMemory =
  HookMemory()

method size*(self: HookMemory): int =
  self.Fsize.map(cb => cb()).get(0)

method get*(self: HookMemory, address: int32): uint8 =
  self.Fget.map(cb => cb(address)).get(0)

method set*(self: HookMemory, address: int32, value: uint8) =
  discard self.Fset.map(cb => (cb(address, value); true))

proc `size=`*(self: HookMemory, h: SizeHook) =
  self.Fsize = some[SizeHook](h).filter(h => h != nil)

proc `get=`*(self: HookMemory, h: GetHook) =
  self.Fget = some[GetHook](h).filter(h => h != nil)

proc `set=`*(self: HookMemory, h: SetHook) =
  self.Fset = some[SetHook](h).filter(h => h != nil)
