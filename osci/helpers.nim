from lists import DoublyLinkedList

##[
  =======
  Helpers
  =======

  The ``Helpers`` module augments the standard library’s modules.
]##


proc length*(dll: DoublyLinkedList): int =
  var
    i = 0
    node = dll.head
  while node != nil:
    inc i
    node = node.next
  return i
