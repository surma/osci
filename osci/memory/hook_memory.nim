from lists import DoublyLinkedList,DoublyLinkedNode,nodes,append
from tables import Table
import options
from future import `->`, `=>`

type
  SetHook* = proc (a: uint32, b: uint8)
  GetHook* = (uint32) -> uint8
  SizeHook* = () -> int
  HookMemory* = ref object of Memory
    ##[
      ``HookMemory``
    ]##
    Fget: Option[GetHook]
    Fset: Option[SetHook]
    Fsize: Option[SizeHook]

proc newHookMemory*(): HookMemory =
  HookMemory()

method size*(pm: HookMemory): int =
  pm.Fsize.map(cb => cb()).get(0)

method get*(pm: HookMemory, address: uint32): uint8 =
  pm.Fget.map(cb => cb(address)).get(0)

proc valueHack(h: SetHook, address: uint32, value: uint8): bool =
  h(address, value)
  true

method set*(pm: HookMemory, address: uint32, value: uint8) =
  discard pm.Fset.map(cb => valueHack(cb, address, value))

proc `size=`*(pm: HookMemory, h: SizeHook) =
  pm.Fsize = some[SizeHook](h)

proc `get=`*(pm: HookMemory, h: GetHook) =
  pm.Fget = some[GetHook](h)

proc `set=`*(pm: HookMemory, h: SetHook) =
  pm.Fset = some[SetHook](h)
