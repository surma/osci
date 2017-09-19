import tables
import options

type
  Symbol* = ref object of RootObject
    name: string

  SymbolTable* = Table[string, Symbol]

proc newSymbolTable*(): SymbolTable =
  return SymbolTable(Table[string, Symbol]())

proc get(self: SymbolTable, symbol: string): Option[Symbol] =
  if not self.hasKey(symbol):
    return none(Symbol)
  some(self.get(symbol))
