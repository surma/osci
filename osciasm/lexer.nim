import pegs
import tables
import options
import hashes
from strutils import `format`

type
  TokenType* = enum colon, dotIdent, newline, ident, number, str
  TokenPosition* = tuple[offset: int, line: int, col: int]
  Token* = ref object of RootObj
    typ: TokenType
    pos: TokenPosition
    value: string

proc newTokenPosition(offset: int, line: int, col: int): TokenPosition =
  (offset: offset, line: line, col: col)

# proc hash(p: Peg): Hash =
#   hash($p)

# let
#   tokenMatchers = {
#     peg"^{\s*}": (none(TokenType), 0),
#     peg"^:": (some(colon), 1),
#     peg"^'.' {[a-zA-Z0-9]*}": (some(dotIdent), 1),
#   }
#   tokenMatcherTable = tokenMatchers.toTable()

iterator tokenize*(s: string): Token =
  var
    buf = s
    offset = 0
    line = 1
    col = 0
    matchlen = 0;
  while buf.len > 0:
    # <colon>
    if buf =~ peg"""^':'""":
      matchlen = 1
      yield Token(typ: colon, pos: newTokenPosition(offset, line, col), value: matches[0])
    # <dotIdent>
    elif buf =~ peg"""^'.'{[a-zA-Z0-9]+}""":
      matchlen = matches[0].len + 1
      yield Token(typ: dotIdent, pos: newTokenPosition(offset, line, col), value: matches[0])
    # <ident>
    elif buf =~ peg"""^{[$a-zA-Z][a-zA-Z0-9_-]*}""":
      matchlen = matches[0].len
      yield Token(typ: ident, pos: newTokenPosition(offset, line, col), value: matches[0])
    # <number>
    elif buf =~ peg"""^{'0x' [0-9a-fA-F]+ / '0b' [0-1]+ / '0o' [0-7]+ / [0-9]+}""":
      matchlen = matches[0].len
      yield Token(typ: number, pos: newTokenPosition(offset, line, col), value: matches[0])
    # <str>
    elif buf =~ peg"""^\"{([^\\\"] / '\\'_)*}\"""":
      matchlen = matches[0].len + 2
      yield Token(typ: str, pos: newTokenPosition(offset, line, col), value: matches[0])
    # <newline>
    elif buf =~ peg"""^{\n}""":
      matchlen = matches[0].len
      yield Token(typ: newline, pos: newTokenPosition(offset, line, col))
      line += 1
      col = 0
    # Any whitespace between tokens
    elif buf =~ peg"""^{(!\n\s)+}""":
      matchlen = matches[0].len
    else:
      raise newException(ValueError, "Unknown token at line $1, column $2 (offset $3): \"$4\"".format(line, col, offset, buf.substr(0, 10)))

    offset += matchlen
    col += matchlen
    buf = buf.substr(matchlen)
