from lists import DoublyLinkedList, DoublyLinkedNode
import options
from future import `->`, `=>`
import macros

## =======
## Helpers
## =======
##
## The ``Helpers`` module augments the standard library’s modules.

proc length*(self: DoublyLinkedList): int =
  ## Returns the length of a ``DoublyLinkedList``
  var
    i = 0
    node = self.head
  while node != nil:
    inc i
    node = node.next
  return i


proc findNodeWithPredicate*[T](self: var DoublyLinkedList[T], pred: (T) -> bool): Option[DoublyLinkedNode[T]] =
  ## Find the first node in a ``DoublyLinkedList`` that matches the given predicate.
  var
    node = self.head
  while node != nil:
    if pred(node.value):
      return some(node)
    node = node.next
  none(DoublyLinkedNode[T])

proc findWithPredicate*[T](self: var DoublyLinkedList[T], pred: (T) -> bool): Option[T] =
  ## Same as ``findNodeWithPredicate``, but returns the value instead of the node.
  self.findNodeWithPredicate(pred).map(node => node.value)

template listItems() =
  var it = self.tail
  while it != nil:
    yield it.value
    it = it.prev

iterator itemsReverse*[T](self: DoublyLinkedList[T]): T =
  ## Yield every value of ``self``, but starts at the back.
  listItems()

iterator mitemsReverse*[T](self: var DoublyLinkedList[T]): T =
  ## Yield every value of ``self`` so that you can modify it, but starts at the back.
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
