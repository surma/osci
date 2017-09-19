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

    check(tokenList == @[
      Token(typ: dotIdent, pos: (line: 1, col: 4), value: "include"),
      Token(typ: str, pos: (line: 1, col: 13), value: "std.asm"),
      Token(typ: newline, pos: (line: 1, col: 22), value: nil),
    ])

    input = """
    my_label:
    .utf8 "Something \"with\" quotes"
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: ident, pos: (line: 1, col: 4), value: "my_label"),
      Token(typ: colon, pos: (line: 1, col: 12), value: nil),
      Token(typ: newline, pos: (line: 1, col: 13), value: nil),
      Token(typ: dotIdent, pos: (line: 2, col: 4), value: "utf8"),
      Token(typ: str, pos: (line: 2, col: 10), value: "Something \\\"with\\\" quotes"),
      Token(typ: newline, pos: (line: 2, col: 37), value: nil),
    ])

    input = """
    .symbol base 0x80000000

    instr1:
    op1:4 op2:9 $ base
    """
    tokenList = toSeq(tokenize(input))

    check(tokenList == @[
      Token(typ: dotIdent, pos: (line: 1, col: 4), value: "symbol"),
      Token(typ: ident, pos: (line: 1, col: 12), value: "base"),
      Token(typ: number, pos: (line: 1, col: 17), value: "0x80000000"),
      Token(typ: newline, pos: (line: 1, col: 27), value: nil),
      Token(typ: newline, pos: (line: 2, col: 0), value: nil),
      Token(typ: ident, pos: (line: 3, col: 4), value: "instr1"),
      Token(typ: colon, pos: (line: 3, col: 10), value: nil),
      Token(typ: newline, pos: (line: 3, col: 11), value: nil),
      Token(typ: ident, pos: (line: 4, col: 4), value: "op1"),
      Token(typ: colon, pos: (line: 4, col: 7), value: nil),
      Token(typ: number, pos: (line: 4, col: 8), value: "4"),
      Token(typ: ident, pos: (line: 4, col: 10), value: "op2"),
      Token(typ: colon, pos: (line: 4, col: 13), value: nil),
      Token(typ: number, pos: (line: 4, col: 14), value: "9"),
      Token(typ: ident, pos: (line: 4, col: 16), value: "$"),
      Token(typ: ident, pos: (line: 4, col: 18), value: "base"),
      Token(typ: newline, pos: (line: 4, col: 22), value: nil),
    ])
