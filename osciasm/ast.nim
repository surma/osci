import parser
import token
import symboltable
import ../osci/instruction

import options

from sequtils import map
from future import `=>`

type
  NodeType* = enum programNode, asmInstructionNode, cpuInstructionNode
  Node* = ref object of RootObj
    parent: Node
    case typ: NodeType
    of programNode:
      instructions: seq[Node]
    of asmInstructionNode:
      command: string
      parameters: seq[parser.Node]
    of cpuInstructionNode:
      instruction: Instruction

proc newProgramNode(): Node =
  Node(parent: nil, typ: programNode)

proc newAsmInstructionNode(): Node =
  Node(parent: nil, typ: asmInstructionNode)

proc newCpuInstructionNode(): Node =
  Node(parent: nil, typ: cpuInstructionNode)

proc processAsmInstructionNode(n: parser.Node, st: SymbolTable): Node =
  result = newAsmInstructionNode()

proc processCpuInstructionNode(n: parser.Node, st: SymbolTable): Node =
  result = newCpuInstructionNode()

proc processInstructionNode(n: parser.Node, st: SymbolTable): Node =
  assert(n.production == ProductionName.instruction)
  assert(n.children.len >= 1)
  var children = n.children
  var label = none(Symbol)
  if children[0].production == ProductionName.label:
    label = some(newSymbol(children[0].token.get().value, SymbolType.variable))
    children = children[0..1]

  assert(n.children.len == 1)
  case n.production
  of ProductionName.asmInstruction:
    result = processAsmInstructionNode(children[0], st)
  of ProductionName.cpuInstruction:
    result = processCpuInstructionNode(children[0], st)
  else:
    raise newException(AssertionError, "Instruction node with child that is not a CPU instruction or ASM instruction")

proc processProgramNode(n: parser.Node, st: SymbolTable): Node =
  assert(n.production == ProductionName.program)
  result = newProgramNode()
  result.children = n.children.map((n) => processInstructionNode(n, st))


proc generateAST*(n: parser.Node, st: SymbolTable): Node =
  processProgramNode(n, st)
