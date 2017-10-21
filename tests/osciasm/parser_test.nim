include ../../osciasm/parser
import ../../osciasm/token
import unittest
from future import `->`, `=>`

suite "parser":
  test "parse":
    var tree: Node

    tree = parse(@[
      Token(typ: token.dotIdent, pos: (0, 0), value: "addr"),
      Token(typ: token.ident, pos: (0, 0), value: "bios"),
      Token(typ: token.op_sub, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "8"),
      Token(typ: token.op_mul, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "4"),
      Token(typ: token.newline, pos: (0, 0), value: nil),
    ])

    check(tree == newNode("program", none(Token), @[
      newNode("instruction", none(Token), @[
        newNode("asm_instruction", none(Token), @[
          newNode("dot_ident", some(Token(typ: token.dotIdent, pos: (0, 0), value: "addr")), @[]),
          newNode("expr", none(Token), @[
            newNode("sum", none(Token), @[
              newNode("product", none(Token), @[
                newNode("value", none(Token), @[
                  newNode("ident", some(Token(typ: token.ident, pos: (0, 0), value: "bios")), @[])])]),
              newNode("op_sum", some(Token(typ: token.op_sub, pos: (0, 0), value: nil)), @[]),
              newNode("sum", none(Token), @[
                newNode("product", none(Token), @[
                  newNode("value", none(Token), @[
                    newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "8")), @[])]),
                  newNode("op_product", some(Token(typ: token.op_mul, pos: (0, 0), value: nil)), @[]),
                  newNode("product", none(Token), @[
                    newNode("value", none(Token), @[
                      newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "4")), @[]),
                    ]),
                  ]),
                ]),
              ]),
            ]),
          ]),
        ]),
      ]),
    ]))

    tree = parse(@[
      Token(typ: token.label, pos: (0, 0), value: "mylabel"),
      Token(typ: token.number, pos: (0, 0), value: "1"),
      Token(typ: token.number, pos: (0, 0), value: "2"),
      Token(typ: token.number, pos: (0, 0), value: "3"),
      Token(typ: token.lparen, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "4"),
      Token(typ: token.op_add, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "5"),
      Token(typ: token.rparen, pos: (0, 0), value: nil),
      Token(typ: token.op_mul, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "6"),
      Token(typ: token.newline, pos: (0, 0), value: nil),
    ])

    check(tree == newNode("program", none(Token), @[
      newNode("instruction", none(Token), @[
        newNode("label", some(Token(typ: token.label, pos: (0, 0), value: "mylabel")), @[]),
        newNode("cpu_instruction", none(Token), @[
          newNode("expr", none(Token), @[
            newNode("sum", none(Token), @[
              newNode("product", none(Token), @[
                newNode("value", none(Token), @[
                  newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "1")), @[])])])])]),
          newNode("expr", none(Token), @[
            newNode("sum", none(Token), @[
              newNode("product", none(Token), @[
                newNode("value", none(Token), @[
                  newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "2")), @[])])])])]),
          newNode("expr", none(Token), @[
            newNode("sum", none(Token), @[
              newNode("product", none(Token), @[
                newNode("value", none(Token), @[
                  newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "3")), @[])])])])]),
          newNode("expr", none(Token), @[
            newNode("sum", none(Token), @[
              newNode("product", none(Token), @[
                newNode("value", none(Token), @[
                  newNode("sum", none(Token), @[
                    newNode("product", none(Token), @[
                      newNode("value", none(Token), @[
                        newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "4")), @[])])]),
                    newNode("op_sum", some(Token(typ: token.op_add, pos: (0, 0), value: nil)), @[]),
                    newNode("sum", none(Token), @[
                      newNode("product", none(Token), @[
                        newNode("value", none(Token), @[
                          newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "5")), @[])])])])])]),
                newNode("op_product", some(Token(typ: token.op_mul, pos: (0, 0), value: nil)), @[]),
                newNode("product", none(Token), @[
                  newNode("value", none(Token), @[
                    newNode("number", some(Token(typ: token.number, pos: (0, 0), value: "6")), @[]),
                  ]),
                ]),
              ]),
            ]),
          ]),
        ]),
      ]),
    ]))
