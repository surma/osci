include ../../osciasm/symboltable
import unittest
import options

from future import `->`, `=>`
from sequtils import toSeq, map, apply

suite "symboltable":
  test "stores items":
    var t = newSymbolTable()
    var s = newSymbol("mysymbol", SymbolType.variable)
    t[s.name] = s

    check(t[s.name] == s)

  test "handles variables":
    var t = newSymbolTable()
    var s = newSymbol("mysymbol", SymbolType.variable)
    s.value = some[int32](4)
    t[s.name] = s

    check(t[s.name].value.get() == 4)

  test "handles types":
    var t = newSymbolTable()
    var s = newSymbol("mysymbol", SymbolType.asmInstruction)
    s.mangler = some(x => 4)
    t[s.name] = s

    check(t[s.name].value.get()() == 4)
