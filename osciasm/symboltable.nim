import tables
import ast
import options

from future import `->`

type
  SymbolType* = enum asmInstruction, variable
  Symbol* = ref object of RootObj
    name*: string
    case typ*: SymbolType
    of asmInstruction:
      mangler*: Option[(ast.Node) -> void]
    of variable:
      value*: Option[int32]

  SymbolTable* = TableRef[string, Symbol]

proc newSymbol*(name: string, typ: SymbolType): Symbol =
  Symbol(name: name, typ: typ)

proc newSymbolTable*(): SymbolTable =
  newTable[string, Symbol]()

# proc get*[A, B](self: Table[A, B], key: A): Option[B] =
#   if self.hasKey(key):
#     return none(B)
#   some(self.get(key))

