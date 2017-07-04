include ../osci/memory
import unittest

suite "memory":
  setup:
    discard

  teardown:
    discard

  test "constants":
    check(LOL == 4)
