include ../osci/helpers
from lists import append
import unittest

suite "helpers":
  test "DoublyLinkedList.length":
    var
      dll: DoublyLinkedList[int] = lists.initDoublyLinkedList[int]()
    check(dll.length == 0)
    dll.append(0)
    check(dll.length == 1)
    dll.append(1)
    check(dll.length == 2)
    dll.append(2)
    check(dll.length == 3)
