from lists import DoublyLinkedList,DoublyLinkedNode,nodes,append
from options import Option, some, none, get
from ../helpers import length

type
  Mount = tuple
    memory: Memory
    mountPoint: uint32
    size: int

  Sentinel = ref object of Memory

  MappedMemory* = ref object of Memory
    ##[
      ``MappedMemory`` maps multiple ``Memory`` into a single address space.

      A ``Memory`` is mounted at a certain address and is from now on responsible for all reads and
      writes between that address (the “mount point”) and where the mounted memory ends. The read and
      write calls for the responsible ``Memory`` will be given an address *relative* to the mount
      point.

      ::
        |                   Unmapped
        |                  <-------->
        |        mem_a                  mem_b
        |  |--------------|            |------|
        |                               null_mem
        |                            |-------------->
        |  |------------- mapped_mem --------------->
        |  |              |            |      |
        |  0            0x100        0x200  0x280

      For example: ``mapped_mem.get(0x208)`` would yield the same value as ``mem_b.get(0x008)``.
    ]##
    mounts: DoublyLinkedList[Mount]

proc newMappedMemory*(): MappedMemory =
  ## Creates a new ``MappedMemory`` with no mappings
  var mm = MappedMemory(mounts: lists.initDoublyLinkedList[Mount]())
  mm.mounts.append((memory: Sentinel(), mountPoint: 0'u32, size: 0))
  mm.mounts.append((memory: Sentinel(), mountPoint: high(uint32), size: 0))
  return mm

proc mount*(mm: MappedMemory, m: Memory, mountPoint: uint32) =
  ## Mount a given memory at the given address
  let
    mount: Mount =
      (
        memory: m,
        mountPoint: mountPoint,
        size: m.size
      )
  for node in mm.mounts.nodes():
    if node.value.mountPoint <= mountPoint:
      continue
    var newNode: DoublyLinkedNode[Mount] = lists.newDoublyLinkedNode[Mount](mount)
    newNode.prev = node.prev
    newNode.next = node
    node.prev.next = newNode
    node.prev = newNode
    return

proc numMounts*(mm: MappedMemory): int =
  mm.mounts.length - 2

proc memoryAtAddress(mm: MappedMemory, address: uint32): Option[Mount] =
  var node = mm.mounts.tail
  while node != nil:
    if node.value.mountPoint <= address and
        int(node.value.mountPoint) + node.value.size > int(address):
      return some(node.value)
    node = node.prev
  return none(Mount)

method size*(mm: MappedMemory): int =
  int(high(uint32))

method get*(mm: MappedMemory, address: uint32): uint8 =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.get(address - mount.mountPoint)

method set*(mm: MappedMemory, address: uint32, value: uint8) =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.set(address - mount.mountPoint, value)
