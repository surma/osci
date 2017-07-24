from memory import Memory, MappedMemory, isMounted, mount, remount
import instruction
from future import `=>`
import macros

##[
  ========
  Emulator
  ========

  The ``Emulator`` module ties together ``Memory``, ``Instruction`` et al to form an osci machine.
  Things like instruction pointer, mount points and BIOS memory – basically all state – is
  encapsulated in an ``Emulator`` instance.
]##

type
  EmulatorObj = object of RootObj
    Fmemory: MappedMemory
    FmainMemory, FbiosMemory: Memory
    ip*: uint32
  Emulator* = ref EmulatorObj

proc replaceInTree(root: NimNode, key, value: string) =
  case root.kind
  of nnkIdent:
    if $root == key:
      root.ident = `!`($value)
  else:
    for child in root.children:
      replaceInTree(child, key, value)

macro replaceIdent(key, val: string, body: untyped): untyped =
  replaceInTree(body, $key, $val)
  body

template memorySetter(name: string) =
  replaceIdent "%", name:
    proc `%Memory=`*(emu: Emulator, newVal: Memory) =
      emu.Fmemory.remount(emu.`F%Memory`, newVal)
      emu.`F%Memory` = newVal

    proc `%Memory`*(emu: Emulator): Memory =
      emu.`F%Memory`

memorySetter("main")
memorySetter("bios")

proc newEmulator*(mainMemory: Memory = memory.newNullMemory(), biosMemory: Memory = memory.newNullMemory()): Emulator =
  result = Emulator(Fmemory: memory.newMappedMemory(), FmainMemory: mainMemory, FbiosMemory: biosMemory)
  result.Fmemory.mount(memory.newNullMemory(), 0)
  result.Fmemory.mount(result.FmainMemory, 0)
  result.Fmemory.mount(result.FbiosMemory, memory.BIOS_ADDRESS)

proc memory*(emu: Emulator): Memory =
  return emu.Fmemory

proc step*(emu: Emulator) =
  var instr = instruction.fromMemory(emu.memory, emu.ip)
  instr.execute(emu.memory, emu.ip)
