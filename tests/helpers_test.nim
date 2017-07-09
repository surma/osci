include ../osci/helpers
from lists import append
import options
from future import `=>`

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

  test "DoublyLinkedList.findWithPredicate":
    type Entry = tuple[value: int; enabled: bool]
    var
      dll: DoublyLinkedList[Entry] = lists.initDoublyLinkedList[Entry]()
      item: Option[Entry]

    item = dll.findWithPredicate((entry: Entry) => entry.enabled)
    check(item.isNone())

    dll.append((value: 5, enabled: false))
    item = dll.findWithPredicate((entry: Entry) => entry.enabled)
    check(item.isNone())

    dll.append((value: 6, enabled: true))
    item = dll.findWithPredicate((entry: Entry) => entry.enabled)
    check(item.isSome())
    check(item.get().value == 6)
