type
  InstructionObj = object of RootObj
    a: uint32
    b: uint32
    target: uint32
    jmp: uint32
  Instruction* = ref InstructionObj

const LOL2* = 9
