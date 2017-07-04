# Package

version       = "0.0.1"
author        = "Surma"
description   = "Emulator and tools for osci, a derivative of subleq"
license       = "MIT"
binDir        = "build"
bin           = @["osci/cli"]
skipDirs       = @["tools", "tests"]

# Dependencies

requires "nim >= 0.17.0"

# Tasks

task test, "Runs the test suite":
  exec "nim c -r tests/instruction.nim"
  exec "nim c -r tests/memory.nim"
