include ../../osciasm/lexer
import ../../osciasm/symboltable
import ../../osciasm/token
import unittest
import sequtils
from future import `->`, `=>`

import unicode
import strutils

suite "parser":
  test "tokenize":
    var
      input: string
      tokenList: seq[Token]

    input = """
    .include "std.asm"
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: token.dotIdent, pos: (line: 1, col: 4), value: "include"),
      Token(typ: token.str, pos: (line: 1, col: 13), value: "std.asm"),
      Token(typ: token.newline, pos: (line: 1, col: 22), value: nil),
    ])

    input = """
    my_label:
    .utf8 "Something \"with\" quotes"
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: token.label, pos: (line: 1, col: 4), value: "my_label"),
      Token(typ: token.newline, pos: (line: 1, col: 13), value: nil),
      Token(typ: token.dotIdent, pos: (line: 2, col: 4), value: "utf8"),
      Token(typ: token.str, pos: (line: 2, col: 10), value: "Something \\\"with\\\" quotes"),
      Token(typ: token.newline, pos: (line: 2, col: 37), value: nil),
    ])

    input = """
    .symbol base 0x80000000

    instr1:
    op1:4 op2:9 $ base
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: token.dotIdent, pos: (line: 1, col: 4), value: "symbol"),
      Token(typ: token.ident, pos: (line: 1, col: 12), value: "base"),
      Token(typ: token.number, pos: (line: 1, col: 17), value: "0x80000000"),
      Token(typ: token.newline, pos: (line: 1, col: 27), value: nil),
      Token(typ: token.newline, pos: (line: 2, col: 0), value: nil),
      Token(typ: token.label, pos: (line: 3, col: 4), value: "instr1"),
      Token(typ: token.newline, pos: (line: 3, col: 11), value: nil),
      Token(typ: token.label, pos: (line: 4, col: 4), value: "op1"),
      Token(typ: token.number, pos: (line: 4, col: 8), value: "4"),
      Token(typ: token.label, pos: (line: 4, col: 10), value: "op2"),
      Token(typ: token.number, pos: (line: 4, col: 14), value: "9"),
      Token(typ: token.ident, pos: (line: 4, col: 16), value: "$"),
      Token(typ: token.ident, pos: (line: 4, col: 18), value: "base"),
      Token(typ: token.newline, pos: (line: 4, col: 22), value: nil),
    ])

    input = """
    .symbol base 0x80000000 + 4*4
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: token.dotIdent, pos: (line: 1, col: 4), value: "symbol"),
      Token(typ: token.ident, pos: (line: 1, col: 12), value: "base"),
      Token(typ: token.number, pos: (line: 1, col: 17), value: "0x80000000"),
      Token(typ: token.op_add, pos: (line: 1, col: 28), value: nil),
      Token(typ: token.number, pos: (line: 1, col: 30), value: "4"),
      Token(typ: token.op_mul, pos: (line: 1, col: 31), value: nil),
      Token(typ: token.number, pos: (line: 1, col: 32), value: "4"),
      Token(typ: token.newline, pos: (line: 1, col: 33), value: nil),
    ])

    test "tokenize with symboltable":
      var
        input: string
        st = newSymbolTable()

      input = """
      .base (0x80 + bios)
      """
      discard toSeq(tokenize(input, st))

      check(st.len == 2)
      check(st["bios"].typ == variable)
      check(st["base"].typ == asmInstruction)
