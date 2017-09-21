import pegs
import tables
import options
import symboltable
import token
from future import `->`, `=>`

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


iterator tokenize*(s: string, st: Option[SymbolTable]): Token =
  var
    matches: array[pegs.MaxSubpatterns, string]
    value: string
    name: string
    offset = 0

    line = 1
    col = 0
  while offset < s.len:
    # <dotIdent>
    token """^'.'{[a-zA-Z0-9]+}""":
      name = matches[0]
      discard st.map(st => st.mgetOrPut(name, newSymbol(name, asmInstruction)))
      yield Token(typ: token.dotIdent, pos: (line: line, col: col), value: name)
    # <ident>
    token """^{[$a-zA-Z][a-zA-Z0-9_-]*}""":
      name = matches[0]
      discard st.map(st => st.mgetOrPut(name, newSymbol(name, variable)))
      yield Token(typ: token.ident, pos: (line: line, col: col), value: matches[0])
    # <number>
    token """^{'0x' [0-9a-fA-F]+ / '0b' [0-1]+ / '0o' [0-7]+ / [0-9]+}""":
      yield Token(typ: token.number, pos: (line: line, col: col), value: matches[0])
    # <str>
    token """^\"{([^\\\"] / '\\'_)*}\"""":
      yield Token(typ: token.str, pos: (line: line, col: col), value: matches[0])
    # <newline>
    token """^{\n}""":
      yield Token(typ: token.newline, pos: (line: line, col: col), value: nil)
      col = -value.len
      line += 1
    # <colon>
    token """^':'""":
      yield Token(typ: token.colon, pos: (line: line, col: col), value: nil)
    # <op_add>
    token """^'+'""":
      yield Token(typ: token.op_add, pos: (line: line, col: col), value: nil)
    # <op_sub>
    token """^'-'""":
      yield Token(typ: token.op_sub, pos: (line: line, col: col), value: nil)
    # <op_mul>
    token """^'*'""":
      yield Token(typ: token.op_mul, pos: (line: line, col: col), value: nil)
    # <op_div>
    token """^'/'""":
      yield Token(typ: token.op_div, pos: (line: line, col: col), value: nil)
    # <lparen>
    token """^'('""":
      yield Token(typ: token.lparen, pos: (line: line, col: col), value: nil)
    # <rparen>
    token """^')'""":
      yield Token(typ: token.rparen, pos: (line: line, col: col), value: nil)

    # whitespace
    token """^{(!\n\s)+}""":
      discard

iterator tokenize*(s: string): Token =
  for token in tokenize(s, none(SymbolTable)):
    yield token

iterator tokenize*(s: string, st: SymbolTable): Token =
  for token in tokenize(s, some(st)):
    yield token
