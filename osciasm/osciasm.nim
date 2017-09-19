import strutils
import sequtils
import os
import ../osci

#[

.source "std.asm"
.symbol base 0x80000000

labelA:
123 0o777 0b110101 0x123
l1:0 l2:0 l3:0 l4:0
l1+3 base+$

some_string:
.utf8 "ohai"
lost:
.byte 4 8 15 16 23 42
mom:
.include "mom.jpg"

Program = <Instruction>*
Instruction =  (<ASMInstruction> | <CPUInstruction>)? <newline>
ASMInstruction = <label>? <dotIdent> (<str>|<ident>|<number>)+
CPUInstruction = (<label>? <Expr>){4}
Label = <ident> <colon>
<Expr> = <Sum>
Sum = <Product> +-] <Sum>
Product = <Number> [*/] <Product>
Number = <ident> | <number>

<colon> = ':'
<dotIdent> = '.' [a-zA-Z0-9]
<newline> = '\n'
<ident> = [$a-zA-Z][a-zA-Z0-9_-]*
<number> = ('0x' [0-9a-fA-F]+ | '0b' [01]+ | '0o' [0-7]+ | [0-9]+)
<str> = "([^"]|\\.)+"
]#



let params = commandLineParams()
if params.len != 1:
  echo "Usage: osciasm <asm file>"
  quit(1)

let
  asmPath = params[0]
  asmCode = readFile(asmPath)

echo asmCode
