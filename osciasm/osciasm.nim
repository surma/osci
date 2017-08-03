import strutils
import sequtils
import os
import ../osci

let params = commandLineParams()
if params.len != 1:
  echo "Usage: osciasm <asm file>"
  quit(1)

let
  asmPath = params[0]
  asmCode = readFile(asmPath)

echo asmCode
