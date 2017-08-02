import lists
import options
import ../helpers
from future import `=>`

type
  Mount = tuple
    memory: Memory
    mountPoint: int32
    size: int

  Sentinel = ref object of Memory

  MappedMemory* = ref object of Memory
    ## Maps multiple ``Memory`` into a single address space.
    ##
    ## A ``Memory`` is mounted at a certain address and is from now on responsible for all reads and
    ## writes between that address (the “mount point”) and where the mounted memory ends. The read
    ## and write calls for the responsible ``Memory`` will be given an address *relative* to the
    ## mount point. If two memories overlap in their area or responsibility, the memory mounted last
    ## will used.
    ##
    ## ::
    ##   |                Unmapped
    ##   |               <-------->
    ##   |     mem_a                  mem_b
    ##   |--------------|            |------|
    ##   |                            null_mem
    ##   |                         |-------------->
    ##   |------------- mapped_mem --------------->
    ##   |              |            |      |
    ##   0            0x100        0x200  0x280
    ##
    ## For example: ``mapped_mem.get(0x208)`` would yield the same value as ``mem_b.get(0x008)``.
    mounts: DoublyLinkedList[Mount]

proc newMappedMemory*(): MappedMemory =
  ## Creates a new ``MappedMemory`` with no mappings.
  var mm = MappedMemory(mounts: lists.initDoublyLinkedList[Mount]())
  mm.mounts.append((memory: Sentinel(), mountPoint: 0'i32, size: 0))
  mm.mounts.append((memory: Sentinel(), mountPoint: high(int32), size: 0))
  return mm

proc mount*(mm: MappedMemory, m: Memory, mountPoint: int32) =
  ## Mounts a given memory at the given address.
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
  ## Replaces a mounted memory with another, preserving shadowing order.
  for node in mm.mounts.nodes():
    if node.value.memory == oldM:
      node.value.memory = newM
      node.value.size = newM.size
      return

proc unmount*(mm: MappedMemory, m: Memory) =
  ## Unmounts a memory. If a memory is mounted multiple times, one instance will be unmounted.
  discard mm.mounts
    .findNodeWithPredicate(mount => mount.memory == m)
    .map(node => (mm.mounts.remove(node); true))

proc numMounts*(mm: MappedMemory): int =
  ## Returns the number of mounted memories (counting duplicates).
  mm.mounts.length - 2

proc memoryAtAddress(mm: MappedMemory, address: int32): Option[Mount] =
  for mount in mm.mounts.mitemsReverse():
    if mount.mountPoint <= address and
        int(mount.mountPoint) + mount.size > int(address):
      return some(mount)
  return none(Mount)

proc isMounted*(mm: MappedMemory, m: Memory): bool =
  ## Checks if the given memory is mounted somewhere.
  mm.mounts.findWithPredicate(mount => mount.memory == m).isSome()

method size*(mm: MappedMemory): int =
  MAX_SIZE

method get*(mm: MappedMemory, address: int32): uint8 =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.get(address - mount.mountPoint)

method set*(mm: MappedMemory, address: int32, value: uint8) =
  let mount = mm.memoryAtAddress(address).get()
  mount.memory.set(address - mount.mountPoint, value)
