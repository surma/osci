include ../../osciasm/parser
import ../../osciasm/token
import unittest
from future import `->`, `=>`

suite "parser":
  test "parse":
    var tree: ParseTreeNode

    tree = parse(@[
      Token(typ: token.dotIdent, pos: (0, 0), value: "addr"),
      Token(typ: token.ident, pos: (0, 0), value: "bios"),
      Token(typ: token.op_sub, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "8"),
      Token(typ: token.op_mul, pos: (0, 0), value: nil),
      Token(typ: token.number, pos: (0, 0), value: "4"),
      Token(typ: token.newline, pos: (0, 0), value: nil),
    ])

    check(tree == newParseTreeNode("program", none(Token), @[
      newParseTreeNode("instruction", none(Token), @[
        newParseTreeNode("asm_instruction", none(Token), @[
          newParseTreeNode("dot_ident", some(Token(typ: token.dotIdent, pos: (0, 0), value: "addr")), @[]),
          newParseTreeNode("expr", none(Token), @[
            newParseTreeNode("sum", none(Token), @[
              newParseTreeNode("product", none(Token), @[
                newParseTreeNode("value", none(Token), @[
                  newParseTreeNode("ident", some(Token(typ: token.ident, pos: (0, 0), value: "bios")), @[])])]),
              newParseTreeNode("op_sum", some(Token(typ: token.op_sub, pos: (0, 0), value: nil)), @[]),
              newParseTreeNode("sum", none(Token), @[
                newParseTreeNode("product", none(Token), @[
                  newParseTreeNode("value", none(Token), @[
                    newParseTreeNode("number", some(Token(typ: token.number, pos: (0, 0), value: "8")), @[])]),
                  newParseTreeNode("op_product", some(Token(typ: token.op_mul, pos: (0, 0), value: nil)), @[]),
                  newParseTreeNode("product", none(Token), @[
                    newParseTreeNode("value", none(Token), @[
                      newParseTreeNode("number", some(Token(typ: token.number, pos: (0, 0), value: "4")), @[]),
                    ]),
                  ]),
                ]),
              ]),
            ]),
          ]),
        ]),
      ]),
    ]))
