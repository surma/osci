import lists
import options
import ../helpers
from future import `=>`

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
        |                Unmapped
        |               <-------->
        |     mem_a                  mem_b
        |--------------|            |------|
        |                            null_mem
        |                         |-------------->
        |------------- mapped_mem --------------->
        |              |            |      |
        0            0x100        0x200  0x280

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

proc remount*(mm: MappedMemory, oldM, newM: Memory) =
  ## Replace a mounted memory with another
  for node in mm.mounts.nodes():
    if node.value.memory == oldM:
      node.value.memory = newM
      node.value.size = newM.size
      return

proc unmount*(mm: MappedMemory, m: Memory) =
  discard mm.mounts
    .findNodeWithPredicate(mount => mount.memory == m)
    .map(node => (mm.mounts.remove(node); true))

proc numMounts*(mm: MappedMemory): int =
  mm.mounts.length - 2

proc memoryAtAddress(mm: MappedMemory, address: uint32): Option[Mount] =
  for mount in mm.mounts.mitemsReverse():
    if mount.mountPoint <= address and
        int(mount.mountPoint) + mount.size > int(address):
      return some(mount)
  return none(Mount)

proc isMounted*(mm: MappedMemory, m: Memory): bool =
  mm.mounts.findWithPredicate(mount => mount.memory == m).isSome()

method size*(mm: MappedMemory): int =
  MAX_SIZE

method get*(mm: MappedMemory, address: uint32): uint8 =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.get(address - mount.mountPoint)

method set*(mm: MappedMemory, address: uint32, value: uint8) =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.set(address - mount.mountPoint, value)
