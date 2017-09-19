include ../../osciasm/lexer
import unittest
from future import `->`, `=>`
from sequtils import toSeq, map, apply

import unicode
import strutils

suite "parser":
  test "tokens":
    var
      input: string
      tokenList: seq[Token]

    input = """
    .include "std.asm"
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList.map(t => (t.typ, t.value)) == @[
      (dotIdent, "include"),
      (str, "std.asm"),
      (newline, nil),
    ])

    input = """
    my_label:
    .utf8 "Something \"with\" quotes"
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList.map(t => (t.typ, t.value)) == @[
      (ident, "my_label"),
      (colon, nil),
      (newline, nil),
      (dotIdent, "utf8"),
      (str, "Something \\\"with\\\" quotes"),
      (newline, nil),
    ])

    input = """
    .symbol base 0x80000000

    instr1:
    op1:4 op2:9 $ base
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList.map(t => (t.typ, t.value)) == @[
      (dotIdent, "symbol"),
      (ident, "base"),
      (number, "0x80000000"),
      (newline, nil),
      (newline, nil),
      (ident, "instr1"),
      (colon, nil),
      (newline, nil),
      (ident, "op1"),
      (colon, nil),
      (number, "4"),
      (ident, "op2"),
      (colon, nil),
      (number, "9"),
      (ident, "$"),
      (ident, "base"),
      (newline, nil),
    ])
