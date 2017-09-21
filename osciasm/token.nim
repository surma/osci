from strutils import `format`

type
  TokenType* = enum label, dotIdent, newline, ident, number, str, op_add, op_sub, op_mul, op_div, lparen, rparen
  TokenPosition* = tuple[line: int, col: int]
  Token* = ref object of RootObj
    typ*: TokenType
    pos*: TokenPosition
    value*: string

proc `==`*(a, b: Token): bool =
  a.typ == b.typ and a.pos == b.pos and a.value == b.value

proc `$`*(t: Token): string =
  "Token(typ: $1, pos: $2, value: \"$3\")".format($t.typ, $t.pos, if t.value == nil: "nil" else: $t.value)
