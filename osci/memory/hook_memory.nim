from lists import DoublyLinkedList,DoublyLinkedNode,nodes,append
from tables import Table
import options
from future import `->`, `=>`

type
  SetHook* = (a: uint32, b: uint8) -> void
  GetHook* = (uint32) -> uint8
  SizeHook* = () -> int
  HookMemory* = ref object of Memory
    ## Externalizes the interface as callbacks (or “hooks”). ``get`` and ``size`` will default to
    ## ``0`` if no hook has been set.
    Fget: Option[GetHook]
    Fset: Option[SetHook]
    Fsize: Option[SizeHook]

proc newHookMemory*(): HookMemory =
  HookMemory()

method size*(pm: HookMemory): int =
  pm.Fsize.map(cb => cb()).get(0)

method get*(pm: HookMemory, address: uint32): uint8 =
  pm.Fget.map(cb => cb(address)).get(0)

method set*(pm: HookMemory, address: uint32, value: uint8) =
  discard pm.Fset.map(cb => (cb(address, value); true))

proc `size=`*(pm: HookMemory, h: SizeHook) =
  pm.Fsize = some[SizeHook](h).filter(h => h != nil)

proc `get=`*(pm: HookMemory, h: GetHook) =
  pm.Fget = some[GetHook](h).filter(h => h != nil)

proc `set=`*(pm: HookMemory, h: SetHook) =
  pm.Fset = some[SetHook](h).filter(h => h != nil)
