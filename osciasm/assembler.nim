import parser
import options

from future import `->`, `=>`
from strutils import format, join
from sequtils import map, toSeq

proc assemble*(prog: parser.Node): seq[byte] =
  @[]
