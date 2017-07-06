from lists import DoublyLinkedList,DoublyLinkedNode,nodes
from options import Option, some, none, get

type
  Mount = tuple
    memory: Memory
    mountPoint: uint32
    size: int

  MappedMemory* = object of Memory
    ##[
      ``MappedMemory`` maps multiple ``Memory``s into a single address space.

      A ``Memory`` is mounted at a certain address and is from now on responsible for all reads and
      writes between that address (the “mount point”) and where the mounted memory ends. The read and
      write calls for the responsible ``Memory`` will be given an address *relative* to the mount
      point.

      ::
                          Unmapped
                        <-------->
              mem_a                   mem_b
        |--------------|            |------|
                                      NullMemory
                                  |-------------->
        |------------- mapped_mem --------------->
        |              |            |      |
        0            0x100        0x200  0x280

      For example: ``mapped_mem.get(0x208)`` would yield the same value as ``mem_b.get(0x008)``.
    ]##
    mounts: DoublyLinkedList[Mount]

proc newMappedMemory*(): MappedMemory =
  ## Creates a new ``MappedMemory`` with no mappings
  MappedMemory(mounts: lists.initDoublyLinkedList[Mount]())

proc mount(mm: var MappedMemory, m: var Memory, mountPoint: uint32) =
  let
    mount: Mount =
      (
        memory: m,
        mountPoint: mountPoint,
        size: m.size
      )
  if mm.mounts.head == nil:
    lists.append(mm.mounts, mount)
    return

  for node in mm.mounts.nodes():
    if node.value.mountPoint < mountPoint:
      continue
    var
      newNode: DoublyLinkedNode[Mount] =
        lists.newDoublyLinkedNode[Mount](mount)
    newNode.prev = node.prev
    newNode.next = node.next
    node.prev.next = newNode
    node.prev = newNode

proc memoryAtAddress(mm: var MappedMemory, address: uint32): Option[Mount] =
  var node = mm.mounts.tail
  while node != nil:
    if node.value.mountPoint <= address and
        node.value.mountPoint + uint32(node.value.size) > address:
      return some(node.value)
    node = node.prev
  return none(Mount)

method size(mm: var MappedMemory): int =
  int(high(uint32))

method get(mm: var MappedMemory, address: uint32): uint32 =
  0

method set(mm: var MappedMemory, address: uint32, value: uint32) =
  discard
