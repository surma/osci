#ifndef _INSTRUCTION_H
#define _INSTRUCTION_H

#include "osciemu/memory.h"

namespace osciemu {
  /**
   * `Instruction` contains a single instruction.
   *
   * An instruction consists of 4 words á 4 bytes. Each instruction
   * can be read as
   *
   * ```
   *   *target := *operand_a - *operand_b
   *   if (*target <= 0)
   *     GOTO jmp;
   * ```
   *
   * Osci is a 32-bit little endian CPU and instructions must be serialized accordingly.
   */
  class Instruction {
    public:
      /**
       * Constructor.
       * Initializes all parameters to zero.
       */
      Instruction();
      /**
       * Constructor.
       * Initializes all parameteres as given.
       * @param a Initial value for `operand_a`
       * @param b Initial value for `operand_b`
       * @param t Initial value for `target`
       * @param j Initial value for `jmp`
       */
      Instruction(uint32_t a, uint32_t b, uint32_t t, uint32_t j);

      uint32_t operand_a;
      uint32_t operand_b;
      uint32_t target;
      uint32_t jmp;

      /**
       * `WriteToMemory` serializes the instruction to memory `m`
       * starting at address `addr`.
       * @param addr Address to start writing at
       * @param m Memory to write serialized instruction to
       * @throws std::out_of_range If write operation is invalid
       */
      void WriteToMemory(MemoryInterface& m, uint32_t addr) const;
      /**
       * `ReadFromMemory` is a constructor that deserializes an instruction
       * from memory `m` starting the read at `addr`.
       * @param addr Address to start reading at
       * @param m Memory to read instruction from
       * @throws std::out_of_range If read operation is invalid
       */
      static Instruction ReadFromMemory(MemoryInterface& m, uint32_t addr);
      /**
       * `Execute` executes the instruction on Memory `m`.
       * This operation will both read and write to Memory `m` and
       * adjust the instruction poiner `ip` accordingly.
       * @param ip Address of instruction to execute
       * @param m Memory to operate on
       * @throws std::out_of_range If read or write is invalid
       */
      static void Execute(MemoryInterface& m, uint32_t& ip);

      friend bool operator==(const Instruction& lhs, const Instruction& rhs);
      friend bool operator!=(const Instruction& lhs, const Instruction& rhs);
  };
}

#endif // _INSTRUCTION_H
