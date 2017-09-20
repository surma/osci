import pegs
import tables
import options
import hashes
from future import `->`, `=>`
from strutils import `format`

type
  TokenType* = enum colon, dotIdent, newline, ident, number, str, op_add, op_sub, op_mul, op_div, lparen, rparen
  TokenPosition* = tuple[line: int, col: int]
  Token* = ref object of RootObj
    typ: TokenType
    pos: TokenPosition
    value: string

proc `==`*(a, b: Token): bool =
  a.typ == b.typ and a.pos == b.pos and a.value == b.value

proc `$`*(t: Token): string =
  "Token(typ: $1, pos: $2, value: \"$3\")".format($t.typ, $t.pos, if t.value == nil: "nil" else: $t.value)

template token(patternStr: string, body: untyped): untyped =
  let
    pattern = peg(patternStr)
    matchLen = s.matchLen(pattern, matches, offset)
  if matchLen >= 0:
    value = s.substr(offset, offset + matchLen - 1)
    offset += matchLen
    when true:
      body
    col += value.len

iterator tokenize*(s: string): Token =
  var
    matches: array[pegs.MaxSubpatterns, string]
    value: string
    offset = 0

    line = 1
    col = 0
  while offset < s.len:
    # <dotIdent>
    token """^'.'{[a-zA-Z0-9]+}""":
      yield Token(typ: dotIdent, pos: (line: line, col: col), value: matches[0])
    # <ident>
    token """^{[$a-zA-Z][a-zA-Z0-9_-]*}""":
      yield Token(typ: ident, pos: (line: line, col: col), value: matches[0])
    # <number>
    token """^{'0x' [0-9a-fA-F]+ / '0b' [0-1]+ / '0o' [0-7]+ / [0-9]+}""":
      yield Token(typ: number, pos: (line: line, col: col), value: matches[0])
    # <str>
    token """^\"{([^\\\"] / '\\'_)*}\"""":
      yield Token(typ: str, pos: (line: line, col: col), value: matches[0])
    # <newline>
    token """^{\n}""":
      yield Token(typ: newline, pos: (line: line, col: col), value: nil)
      col = -value.len
      line += 1
    # <colon>
    token """^':'""":
      yield Token(typ: colon, pos: (line: line, col: col), value: nil)
    # <op_add>
    token """^'+'""":
      yield Token(typ: op_add, pos: (line: line, col: col), value: nil)
    # <op_sub>
    token """^'-'""":
      yield Token(typ: op_sub, pos: (line: line, col: col), value: nil)
    # <op_mul>
    token """^'*'""":
      yield Token(typ: op_mul, pos: (line: line, col: col), value: nil)
    # <op_div>
    token """^'/'""":
      yield Token(typ: op_div, pos: (line: line, col: col), value: nil)
    # <lparen>
    token """^'('""":
      yield Token(typ: lparen, pos: (line: line, col: col), value: nil)
    # <rparen>
    token """^')'""":
      yield Token(typ: rparen, pos: (line: line, col: col), value: nil)

    # whitespace
    token """^{(!\n\s)+}""":
      discard
