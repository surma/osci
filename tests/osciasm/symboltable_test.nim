include ../../osciasm/symboltable
import unittest
from future import `->`, `=>`
from sequtils import toSeq, map, apply

suite "symboltable":
  test "stores items":
    var t = newSymbolTable()
    var s = newSymbol("mysymbol", variable)
    t[s.name] = s

    check(t[s.name] == s)
