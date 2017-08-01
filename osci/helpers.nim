from lists import DoublyLinkedList, DoublyLinkedNode
import options
from future import `->`, `=>`
import macros

## =======
## Helpers
## =======
##
## The ``Helpers`` module augments the standard library’s modules.

proc length*(dll: DoublyLinkedList): int =
  ## Returns the length of a ``DoublyLinkedList``
  var
    i = 0
    node = dll.head
  while node != nil:
    inc i
    node = node.next
  return i


proc findNodeWithPredicate*[T](dll: var DoublyLinkedList[T], pred: (T) -> bool): Option[DoublyLinkedNode[T]] =
  ## Find the first node in a ``DoublyLinkedList`` that matches the given predicate.
  var
    node = dll.head
  while node != nil:
    if pred(node.value):
      return some(node)
    node = node.next
  none(DoublyLinkedNode[T])

proc findWithPredicate*[T](dll: var DoublyLinkedList[T], pred: (T) -> bool): Option[T] =
  ## Same as ``findNodeWithPredicate``, but returns the value instead of the node.
  dll.findNodeWithPredicate(pred).map(node => node.value)

template listItems() =
  var it = dll.tail
  while it != nil:
    yield it.value
    it = it.prev

iterator itemsReverse*[T](dll: DoublyLinkedList[T]): T =
  ## Yield every value of ``dll``, but starts at the back.
  listItems()

iterator mitemsReverse*[T](dll: var DoublyLinkedList[T]): T =
  ## Yield every value of ``dll`` so that you can modify it, but starts at the back.
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
  ## A macro the replaces all occurences of an identifer in the given body expression.
  replaceInTree(body, $key, $val)
  body
