from lists import DoublyLinkedList
import options
from future import `->`

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

proc findWithPredicate*[T](dll: var DoublyLinkedList[T], pred: (T) -> bool): Option[T] =
  var
    node = dll.head
  while node != nil:
    if pred(node.value):
      return some[T](node.value)
    node = node.next
  none(T)

iterator itemsReverse*[T](dll: var DoublyLinkedList[T]): T =
  var it = dll.tail
  while it != nil:
    yield it.value
    it = it.prev
