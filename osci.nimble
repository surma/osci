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
  exec "nim c -r tests/osci/helpers_test.nim"
  exec "nim c -r tests/osci/memory_test.nim"
  exec "nim c -r tests/osci/instruction_test.nim"
  exec "nim c -r tests/osci/emulator_test.nim"
  exec "nim c -r tests/osciasm/lexer_test.nim"
  exec "nim c -r tests/osciasm/symboltable_test.nim"
