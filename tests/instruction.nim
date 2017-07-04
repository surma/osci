include ../osci/instruction
import unittest

suite "instruction":
  setup:
    discard

  teardown:
    discard

  test "constants":
    check(LOL2 == 9)
