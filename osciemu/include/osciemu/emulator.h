#ifndef _EMULATOR_H
#define _EMULATOR_H

#include "osciemu/memory.h"
#include "osciemu/instruction.h"

#define CEIL(x, y) ((x) + (y) - 1)/(y)

namespace osciemu {
  /**
   * `Emulator` is a class wiring up memory and instruction interpreter
   * to behave like an osci CPU.
   *
   * At the start, the BIOS memory is mapped to the address space
   * at 2^31 and the instruction pointer (IP) is set to 2^31.
   *
   * The end of the virtual memory is an area of mapped memory for
   * control flags, peripherals and interrupts.
   *
   * ```
   * +---------------------------------------+ Address 0
   * |                 Word 0                |
   * |                 Word 1                |
   * |                   ...                 |
   * +---------------------------------------+ kFlagBoundary
   * |              Flags Word 0             |
   * |                   ...                 |
   * |              Flags Word i             |
   * +---------------------------------------+ kIvtBoundary
   * |               IVT Entry 0             |
   * |                   ...                 |
   * |               IVT Entry j             |
   * +---------------------------------------+ kRegisterBoundary
   * |                Register 0             |
   * |                   ...                 |
   * |                Register k             |
   * +---------------------------------------+ kMaxAddress
   * ```
   *
   * A word is 4 byte in little endian. osci CPU always has
   * 2^32 bytes of virtual memory.
   * Not all the memory is necessarily backed by physical
   * memory. Reads from purely virtual memory yiel 0. Writes
   * to purely virtual memory are ignored.
   *
   * Flags Word 0:
   *
   * ```
   * +-----------------------------------+---+
   * |    |    |    |    |    |    | bD | H  | Byte 0
   * +---------------------------------------+
   * |                 Unused                |
   * +---------------------------------------+
   * |                 Unused                |
   * +---------------------------------------+
   * |                 Unused                | Byte 3
   * +---------------------------------------+
   * ```
   *
   * * biosDone (bD): Unmaps the BIOS from the address space
   * * halt (H): Halts the CPU
   *
   */
  class Emulator : public MemoryInterface {
    public:
      /**
       * Constructor.
       * @param main Main memory of the CPU
       * @param bios Bios memory
       */
      Emulator(MemoryInterface& main, MemoryInterface& bios);
      ~Emulator();
      /**
       * `Step` executes the next instruction of the osci CPU.
       */
      void Step();

      /**
       *
       * @see MemoryInterface::GetSize
       */
      uint32_t GetSize() const;
      /**
       * @see MemoryInterface::GetCell
       */
      uint8_t GetCell(uint32_t addr) const;
      /**
       * @see MemoryInterface::SetCell
       */
      void SetCell(uint32_t addr, uint8_t value);

      /**
       * `SetBiosMap` maps or unmaps the BIOS
       * @param state true if the BIOS should be mapped
       */
      void SetBiosMap(bool newState);
      /**
       * `IsHalted` checks if the halted bit (H) is set.
       */
      bool IsHalted() const;

      static const uint32_t kBiosBoundary = 1<<31;
      static const uint32_t kMaxAddress = 0xFFFFFFFF;
      static const uint8_t kNumRegisters = 4;
      static const uint8_t kNumIvts = 1;
      static const uint8_t kNumFlags = 1;
      static const uint32_t kRegisterBoundary = kMaxAddress - kNumRegisters*Instruction::Word;
      static const uint32_t kIvtBoundary = kRegisterBoundary - kNumIvts*Instruction::Word;
      static const uint32_t kFlagBoundary = kIvtBoundary - CEIL(kNumFlags, Instruction::Word*8)*Instruction::Word;
      static const uint32_t kControlBoundary = kFlagBoundary;
      uint32_t ip_;


    private:
      void ProcessFlagChanges();
      inline uint32_t GetFlagByteAddress(uint8_t flag, uint8_t byte) const;

      ZeroMemory memory_;
      MappedMemory mappedMemory_;
      MemoryInterface& biosMemory_;
      ArrayMemory controlMemory_;
  };
}

#endif // _EMULATOR_H
