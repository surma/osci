import memory

## =================
## CPU instructions
## =================
##
## An instruction consists of 4 words á 4 bytes. Each instruction is a set of 4 addresses ``op_a``,
## ``op_b``, ``target`` and ``jmp``. All 4 words are *signed* words. If a word is negative, it is
## considered indirect. The execution of an instruction is described by the following pseudo-code::
##
##   if(op_a < 0) op_a = *(-op_a)
##   if(op_b < 0) op_b = *(-op_b)
##   if(target < 0) target = *(-target)
##   if(jmp < 0) jmp = *(-jmp)
##
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

proc serialize*(self: Instruction, m: Memory, address: int32) =
  ## Serializes (writes) an instruction to memory.
  m.writeInt32(address + 00, self.op_a)
  m.writeInt32(address + 04, self.op_b)
  m.writeInt32(address + 08, self.target)
  m.writeInt32(address + 12, self.jmp)

proc deserialize*(self: Instruction, m: Memory, address: int32) =
  ## Deserializes (reads) an instruction from memory.
  self.op_a = m.readInt32(address + 00)
  self.op_b = m.readInt32(address + 04)
  self.target = m.readInt32(address + 08)
  self.jmp = m.readInt32(address + 12)

proc fromMemory*(m: Memory, address: int32): Instruction =
  ## Creates a new instruction and initializes it with the values at the given memory location.
  result = newInstruction()
  result.deserialize(m, address)

proc execute*(self: Instruction, m: Memory, ip: var int32) =
  ## Executes the instruction on the given memory.
  var
    op_a = self.op_a
    op_b = self.op_b
    target = self.target
    jmp = self.jmp
  if op_a < 0: op_a = m.readInt32(-op_a)
  if op_b < 0: op_b = m.readInt32(-op_b)
  if target < 0: target = m.readInt32(-target)
  if jmp < 0: jmp = m.readInt32(-jmp)
  let result = m.readInt32(op_a) - m.readInt32(op_b)
  m.writeInt32(target, result)
  if result <= 0:
    # Round up to the next word boundary
    ip = int32(int((jmp + WORD_SIZE - 1) / WORD_SIZE) * WORD_SIZE)
  else:
    ip += INSTRUCTION_SIZE
