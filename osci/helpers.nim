from lists import DoublyLinkedList
import options
from future import `->`
import macros

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

template listItems() =
  var it = dll.tail
  while it != nil:
    yield it.value
    it = it.prev

iterator itemsReverse*[T](dll: DoublyLinkedList[T]): T =
  listItems()

iterator mitemsReverse*[T](dll: var DoublyLinkedList[T]): T =
  listItems()

proc replaceInTree(root: NimNode, key, value: string) =
  case root.kind
  of nnkIdent:
    if $root == key:
      root.ident = `!`($value)
  else:
    for child in root.children:
      replaceInTree(child, key, value)

macro replaceIdent*(key, val: string, body: untyped): untyped =
  replaceInTree(body, $key, $val)
  body

proc or_else*[T](self: Option[T], def: T): T =
  result = def
  if(self.isSome()):
    result = self.get()
