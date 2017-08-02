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
    FmainMemory: Memory
    FbiosMemory: Memory
    FreadonlyBiosMemory: ReadonlyMemory
    FregisterMemory: ArrayMemory
    ip*: int32
    halted*: bool
    FbiosDone: bool
  Emulator* = ref EmulatorObj

proc `mainMemory=`*(self: Emulator, newVal: Memory) =
  self.Fmemory.remount(self.FmainMemory, newVal)
  self.FmainMemory = newVal

proc mainMemory*(self: Emulator): Memory =
  self.FmainMemory

proc `biosMemory=`*(self: Emulator, newVal: Memory) =
  self.FbiosMemory = newVal
  let n = newReadonlyMemory(self.FbiosMemory)
  self.Fmemory.remount(self.FreadonlyBiosMemory, n)
  self.FreadonlyBiosMemory = n


proc biosMemory*(self: Emulator): Memory =
  self.FbiosMemory

proc biosDone*(self: Emulator): bool =
  self.FbiosDone

proc `biosDone=`*(self: Emulator, done: bool) =
  if done != self.FbiosDone and not done:
    self.Fmemory.remount(emptyBiosMemory, self.FreadonlyBiosMemory)
  if done != self.FbiosDone and done:
    self.Fmemory.remount(self.FreadonlyBiosMemory, emptyBiosMemory)
  self.FbiosDone = done

proc flagSet(self: Emulator, address: int32, value: uint8) =
  case address
  of 0*4 + 0:
    self.halted = ((value shr FLAG_HALT) and 1) == 1
    self.biosDone = ((value shr FLAG_BIOS_DONE) and 1) == 1
  else:
    discard

proc flagGet(self: Emulator, address: int32): uint8 =
  case address
  of 0:
    result = (uint8(self.halted) shl FLAG_HALT) or (uint8(self.biosDone) shl FLAG_BIOS_DONE)
  else:
    result = 0

proc newEmulator*(mainMemory: Memory = newNullMemory(), biosMemory: Memory = newNullMemory()): Emulator =
  var r: Emulator = Emulator(
    Fmemory: newMappedMemory(),
    FmainMemory: mainMemory,
    FbiosMemory: biosMemory,
    FreadonlyBiosMemory: newReadonlyMemory(biosMemory),
    FregisterMemory: newArrayMemory(NUM_REGISTERS * WORD_SIZE),
    FflagMemory: newHookMemory(),
    ip: BIOS_ADDRESS
  )

  r.Fmemory.mount(newNullMemory(), 0)
  r.Fmemory.mount(r.FmainMemory, 0)
  r.Fmemory.mount(r.FreadonlyBiosMemory, BIOS_ADDRESS)
  r.halted = false
  r.biosDone = false
  r.FflagMemory.get = (address: int32) => r.flagGet(address)
  r.FflagMemory.set = (address: int32, value: uint8) => r.flagSet(address, value)
  r.FflagMemory.size = () => NUM_FLAG_WORDS
  r.Fmemory.mount(r.FflagMemory, FLAGS0_ADDRESS)
  r.Fmemory.mount(r.FregisterMemory, REGISTER0_ADDRESS)
  r

proc register*(self: Emulator, idx: int): int32 =
  self.FregisterMemory.readInt32(int32(idx * WORD_SIZE))

proc memory*(self: Emulator): Memory =
  return self.Fmemory

proc step*(self: Emulator) =
  ## Executes the current instruction.
  if self.halted: return
  var instr = instruction.fromMemory(self.memory, self.ip)
  instr.execute(self.memory, self.ip)
