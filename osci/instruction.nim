from memory import Memory, writeUint32, readUint32, readInt32, writeInt32

## =================
## CPU instructions
## =================
##
## An instruction consists of 4 words á 4 bytes. Each instruction is a set of 4 addresses ``op_a``,
## ``op_b``, ``target`` and ``jmp``. The execution of an instruction is equivalent to
##
## ::
##   *target := *op_a - *op_b
##   if (*target <= 0)
##     GOTO jmp;
##
## ``jmp`` must be a multiple of the word size. If it’s not, it will be rounded to the next biggest
## multiple of the word size.
##
## osci is a 32-bit little endian CPU and instructions must be serialized accordingly.

const INSTRUCTION_SIZE* = 4 * 4

type
  Instruction* = ref object of RootObj
    op_a: int32
    op_b: int32
    target: int32
    jmp: int32

proc newInstruction*(op_a, op_b, target, jmp: int32 = 0): Instruction =
  ## Creates a new instruction.
  Instruction(op_a: op_a, op_b: op_b, target: target, jmp: jmp)

proc `==`*(instr1, instr2: Instruction): bool =
  ## Equality check for instructions.
  true and
    instr1.op_a == instr2.op_a and
    instr1.op_b == instr2.op_b and
    instr1.target == instr2.target and
    instr1.jmp == instr2.jmp

proc serialize*(instr: Instruction, m: Memory, address: int32) =
  ## Serializes (writes) an instruction to memory.
  m.writeInt32(address + 00, instr.op_a)
  m.writeInt32(address + 04, instr.op_b)
  m.writeInt32(address + 08, instr.target)
  m.writeInt32(address + 12, instr.jmp)

proc deserialize*(instr: Instruction, m: Memory, address: int32) =
  ## Deserializes (reads) an instruction from memory.
  instr.op_a = m.readInt32(address + 00)
  instr.op_b = m.readInt32(address + 04)
  instr.target = m.readInt32(address + 08)
  instr.jmp = m.readInt32(address + 12)

proc fromMemory*(m: Memory, address: int32): Instruction =
  ## Creates a new instruction and initializes it with the values at the given memory location.
  result = newInstruction()
  result.deserialize(m, address)

proc execute*(instr: Instruction, m: Memory, ip: var int32) =
  ## Executes the instruction on the given memory.
  let
    op_a = m.readInt32(instr.op_a)
    op_b = m.readInt32(instr.op_b)
  let result = op_a - op_b
  m.writeInt32(instr.target, result)
  if result < 0:
    ip = instr.jmp
  else:
    ip += INSTRUCTION_SIZE
