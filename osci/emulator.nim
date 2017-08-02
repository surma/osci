import memory
import instruction
from helpers import replaceIdent
from future import `=>`

## ========
## Emulator
## ========
##
## The ``Emulator`` module ties together ``Memory``, ``Instruction`` et al to form an osci machine.
## Things like instruction pointer, mount points and BIOS memory – basically all state – is
## encapsulated in an ``Emulator`` instance.

let emptyBiosMemory = newArrayMemory(@[])

type
  EmulatorObj = object of RootObj
    ## Represents an osci emulator.
    Fmemory: MappedMemory
    FflagMemory: HookMemory
    FmainMemory, FbiosMemory: Memory
    FregisterMemory: ArrayMemory
    ip*: int32
    halted*: bool
    FbiosDone: bool
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

proc biosDone*(emu: Emulator): bool =
  emu.FbiosDone

proc `biosDone=`*(emu: Emulator, done: bool) =
  if done != emu.FbiosDone and not done:
    emu.Fmemory.remount(emptyBiosMemory, emu.biosMemory)
  if done != emu.FbiosDone and done:
    emu.Fmemory.remount(emu.biosMemory, emptyBiosMemory)
  emu.FbiosDone = done

proc flagSet(emu: Emulator, address: int32, value: uint8) =
  case address
  of 0*4 + 0:
    emu.halted = ((value shr FLAG_HALT) and 1) == 1
    emu.biosDone = ((value shr FLAG_BIOS_DONE) and 1) == 1
  else:
    discard

proc flagGet(emu: Emulator, address: int32): uint8 =
  case address
  of 0:
    result = (uint8(emu.halted) shl FLAG_HALT) or (uint8(emu.biosDone) shl FLAG_BIOS_DONE)
  else:
    result = 0

proc newEmulator*(mainMemory: Memory = newNullMemory(), biosMemory: Memory = newNullMemory()): Emulator =
  var r: Emulator = Emulator(
    Fmemory: newMappedMemory(),
    FmainMemory: mainMemory,
    FbiosMemory: biosMemory,
    FregisterMemory: newArrayMemory(NUM_REGISTERS * WORD_SIZE),
    FflagMemory: newHookMemory(),
    ip: BIOS_ADDRESS
  )

  r.Fmemory.mount(newNullMemory(), 0)
  r.Fmemory.mount(r.FmainMemory, 0)
  r.Fmemory.mount(r.FbiosMemory, BIOS_ADDRESS)
  r.halted = false
  r.biosDone = false
  r.FflagMemory.get = (address: int32) => r.flagGet(address)
  r.FflagMemory.set = (address: int32, value: uint8) => r.flagSet(address, value)
  r.FflagMemory.size = () => NUM_FLAG_WORDS
  r.Fmemory.mount(r.FflagMemory, FLAGS0_ADDRESS)
  r.Fmemory.mount(r.FregisterMemory, REGISTER0_ADDRESS)
  r

proc register*(emu: Emulator, idx: int): int32 =
  emu.FregisterMemory.readInt32(int32(idx * WORD_SIZE))

proc memory*(emu: Emulator): Memory =
  return emu.Fmemory

proc step*(emu: Emulator) =
  ## Executes the current instruction.
  if emu.halted: return
  var instr = instruction.fromMemory(emu.memory, emu.ip)
  instr.execute(emu.memory, emu.ip)
