import strutils
import sequtils
import os
import ../osci

proc readFileBuffer(path: string): seq[uint8] =
  let
    file = open(path)
    len = getFileSize(file)
  result = newSeq[uint8](len)
  discard file.readBuffer(addr(result[0]), len)
  close(file)


let params = commandLineParams()
if params.len != 2:
  echo "Usage: osci-cli <bios image> <main image>"
  quit(1)

let
  (biosImagePath, mainImagePath) = (params[0], params[1])
  (biosImage, mainImage) = (readFileBuffer(biosImagePath), readFileBuffer(mainImagePath))
  (biosMemory, mainMemory) = (osci.memory.newArrayMemory(biosImage), osci.memory.newArrayMemory(mainImage))
  emulator = osci.emulator.newEmulator(biosMemory = biosMemory, mainMemory = mainMemory)

iterator stepInfo(emu: Emulator): string =
  yield "ip:"
  yield emu.ip.toHex()
  for i in 0..<osci.memory.NUM_REGISTERS:
    yield "r" & i.intToStr() & ":"
    yield emu.register(i).toHex()

while not emulator.halted:
  echo toSeq(stepInfo(emulator)).join(" ")
  emulator.step()

echo "CPU was halted"
