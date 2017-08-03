# osci

osci is a [OISC]. Inspired by [SUBLEQ], the architecture has been changed and augmented for simplicity and a slight boost in power.

## Architecture overview

osci...

- is a 32-bit, little endian CPU
- always has 2^31 bytes (2GB) of memory, not all of it necessarily writable
- is similar to [SUBLEQ], but has a [separate ``target`` operand and support for indirect addresses][Instructions]
- has a defined [memory layout] with dynamically unmappable BIOS memory
- has configurable interrupts
- ... more?

## Usage

The core of this project is (currently) an emulator of the osci architecture. It is written in [Nim]. The `osci` module contains an emulator implementation as well as all the building blocks of that emulator. Additionally, there is a crude CLI implementation and some tools (that urgently need to be ported to [Nim]).

Example:

```
$ cat examples/simpleloop.bios.hex | ./tools/hexcompile/hexcompile.sh > bios.img
$ echo something > main.img
$ nim c oscicli/oscicli.nim
$ ./oscicli/oscicli ./bios.img ./main.img
```

License
-------
MIT

[OISC]: https://en.wikipedia.org/wiki/One_instruction_set_computer
[SUBLEQ]: https://esolangs.org/wiki/Subleq
[Instructions]: https://github.com/surma/osci/blob/master/osci/instruction.nim
[Memory layout]: https://github.com/surma/osci/blob/master/osci/memory.nim
[Nim]: https://nim-lang.org/
