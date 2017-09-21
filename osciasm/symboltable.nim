import tables
import options

type
  SymbolType* = enum asmInstruction, variable
  Symbol* = ref object of RootObj
    name*: string
    typ*: SymbolType
    value*: Option[string]

  SymbolTable* = TableRef[string, Symbol]

proc newSymbol*(name: string, typ: SymbolType): Symbol =
  Symbol(name: name, typ: typ, value: none(string))

proc newSymbol*(name: string, typ: SymbolType, value: string): Symbol =
  Symbol(name: name, typ: typ, value: some(value))

proc newSymbolTable*(): SymbolTable =
  newTable[string, Symbol]()

# proc get*[A, B](self: Table[A, B], key: A): Option[B] =
#   if self.hasKey(key):
#     return none(B)
#   some(self.get(key))

