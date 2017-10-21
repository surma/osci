import token
import options

from future import `->`, `=>`
from strutils import format, join
from sequtils import map, toSeq

type
  PeekableIterator[T] = ref object of RootObj
    data: seq[T]
    head: int

proc newPeekableIterator[T](data: seq[T]): PeekableIterator[T] =
  PeekableIterator[T](data: data, head: data.low)

proc done(self: PeekableIterator): bool =
  self.head > self.data.high

proc next[T](self: PeekableIterator[T]): T =
  if self.head > self.data.high:
    raise newException(RangeError, "Attempt to read past EOF")
  result = self.data[self.head]
  self.head += 1

proc peek[T](self: PeekableIterator[T]): T =
  self.data[self.head]

template unreachable[T](pit: PeekableIterator[T]): untyped =
  assert(false, "Unreachable. Current token: $1".format($pit.peek()))

type
  Node* = ref object of RootObj
    children: seq[Node]
    production: string
    parent: Node
    token: Option[Token]

proc `==`*(a, b: Node): bool =
  a.children == b.children and a.production == b.production and a.token == b.token

proc newNode*(production: string, token: Option[Token], children: seq[Node]): Node =
  Node(production: production, token: token, children: children)

proc newNode*(production: string, token: Token): Node =
  Node(children: @[], production: production, token: some(token))

proc newNode*(production: string): Node =
  Node(children: @[], production: production, token: none(Token))

proc addChild*(self: Node, c: Node) =
  self.children.add(c)
  c.parent = self

proc `$`*(n: Node): string =
  "($1 [$2])".format(n.production, n.children.map(c => $c).join(", "))

proc assertNext(pit: PeekableIterator[Token], typ: TokenType): Token =
  result = pit.next()
  assert(result.typ == typ)

proc peekIsFirstOfExpr(pit: PeekableIterator[Token]): bool =
  return pit.peek().typ == token.ident or pit.peek().typ == token.number or pit.peek().typ == token.lparen

proc parseSum(pit: PeekableIterator[Token]): Node

proc parseValue(pit: PeekableIterator[Token]): Node =
  result = newNode("value")
  if pit.peek().typ == token.ident:
    result.addChild(newNode("ident", pit.assertNext(token.ident)))
  elif pit.peek().typ == token.number:
    result.addChild(newNode("number", pit.assertNext(token.number)))
  elif pit.peek().typ == token.lparen:
    discard pit.assertNext(token.lparen)
    result.addChild(parseSum(pit))
    discard pit.assertNext(token.rparen)
  else:
    unreachable(pit)

proc parseProduct(pit: PeekableIterator[Token]): Node =
  result = newNode("product")
  result.addChild(parseValue(pit))
  if pit.peek().typ == token.op_mul or pit.peek().typ == token.op_div:
    result.addChild(newNode("op_product", pit.next()))
    result.addChild(parseProduct(pit))

proc parseSum(pit: PeekableIterator[Token]): Node =
  result = newNode("sum")
  result.addChild(parseProduct(pit))
  if pit.peek().typ == token.op_add or pit.peek().typ == token.op_sub:
    result.addChild(newNode("op_sum", pit.next()))
    result.addChild(parseSum(pit))

proc parseExpr(pit: PeekableIterator[Token]): Node =
  result = newNode("expr")
  result.addChild(parseSum(pit))

proc parseCPUInstruction(pit: PeekableIterator[Token]): Node =
  result = newNode("cpu_instruction")
  for i in 0..2:
    result.addChild(parseExpr(pit))
    if pit.peek().typ == token.label:
      result.addChild(newNode("label", pit.assertNext(token.label)))
  result.addChild(parseExpr(pit))
  discard pit.assertNext(token.newline)

proc parseASMInstruction(pit: PeekableIterator[Token]): Node =
  result = newNode("asm_instruction")
  result.addChild(newNode("dot_ident", pit.assertNext(token.dotIdent)))
  while true:
    if pit.peekIsFirstOfExpr():
      result.addChild(parseExpr(pit))
      continue
    if pit.peek().typ == token.str:
      result.addChild(newNode("str", pit.assertNext(token.str)))
      continue
    break
  discard pit.assertNext(token.newline)

proc parseInstruction(pit: PeekableIterator[Token]): Node =
  result = newNode("instruction")
  if pit.peek().typ == token.newline:
    return
  if pit.peek().typ == token.label:
    result.addChild(newNode("label", pit.next()))
  if pit.peek().typ == token.dotIdent:
    result.addChild(parseASMInstruction(pit))
    return
  if pit.peekIsFirstOfExpr():
    result.addChild(parseCPUInstruction(pit))
    return
  unreachable(pit)

proc parseProgram(pit: PeekableIterator[Token]): Node =
  result = newNode("program")
  while not pit.done:
    var exprNode = parseInstruction(pit)
    result.addChild(exprNode)

proc parse*(tokens: seq[Token]): Node =
  parseProgram(newPeekableIterator(tokens))
