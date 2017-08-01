from memory import Memory, MappedMemory, isMounted, mount, remount
import instruction
from helpers import replaceIdent

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
    halted: bool
  Emulator* = ref EmulatorObj

template memorySetter(name: string) =
  replaceIdent "%", name:
    proc `%Memory=`*(emu: Emulator, newVal: Memory) =
      emu.Fmemory.remount(emu.`F%Memory`, newVal)
      emu.`F%Memory` = newVal

    proc `%Memory`*(emu: Emulator): Memory =
      emu.`F%Memory`

memorySetter("main")
memorySetter("bios")

proc newEmulator*(mainMemory: Memory = newNullMemory(), biosMemory: Memory = newNullMemory()): Emulator =
  var r: Emulator = Emulator(
    Fmemory: newMappedMemory(),
    FmainMemory: mainMemory,
    FbiosMemory: biosMemory,
  )

  r.Fmemory.mount(newNullMemory(), 0)
  r.Fmemory.mount(r.FmainMemory, 0)
  r.Fmemory.mount(r.FbiosMemory, BIOS_ADDRESS)
  r.halted = false
  r

proc memory*(emu: Emulator): Memory =
  return emu.Fmemory

proc step*(emu: Emulator) =
  if emu.halted: return
  var instr = instruction.fromMemory(emu.memory, emu.ip)
  instr.execute(emu.memory, emu.ip)

proc isHalted*(emu: Emulator): bool =
  emu.halted
