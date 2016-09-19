#ifndef _EMULATOR_H
#define _EMULATOR_H

#include "osciemu/memory.h"
#include "osciemu/instruction.h"

namespace osciemu {
  /**
   * `Emulator` is a class wiring up memory and instruction interpreter
   * to behave like an osci CPU.
   *
   * At the start, the bios memory is mapped to the address space
   * at 2^31 and the instruction pointer (IP) is set to 2^31. By setting
   * the biosDone (bD) flag to 1, the bios will be unmapped.
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
   * A word is 4 byte. osci CPU always has 2^32 bytes of
   * virtual memory. Not all the memory is necessarily backed
   * by physical memory. Reads from purely virtual memory yield
   * 0. Writes to purely virtual memory are ignored.
   *
   * Flags Word 0:
   *
   * ```
   * +-----------------------------------+---+
   * |    |    |    |    |    |    |    | bD | Byte 0
   * +---------------------------------------+
   * |                 Unused                |
   * +---------------------------------------+
   * |                 Unused                |
   * +---------------------------------------+
   * |                 Unused                | Byte 3
   * +---------------------------------------+
   * ```
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
       * `SetBiosDoneFlag` sets the bD flag to `state`.
       * @param state State to set the bD flag to
       */
      void SetBiosDoneFlag(bool state);

      static const uint32_t kBiosBoundary = 1<<31;
      static const uint32_t kMaxAddress = 0xFFFFFFFF;
      static const uint8_t kNumRegisters = 4;
      static const uint8_t kNumIvts = 1;
      static const uint8_t kNumFlags = 1;
      static const uint32_t kRegisterBoundary = kMaxAddress - kNumRegisters*Instruction::Size;
      static const uint32_t kIvtBoundary = kMaxAddress - (kNumRegisters + kNumIvts)*Instruction::Size;
      static const uint32_t kFlagBoundary = kIvtBoundary - (kNumFlags + 32 - 1)/32; // = kIvtBoundary - ceil(kNumFlags / 32);
      uint32_t ip_;


    private:
      uint8_t FlagRead(uint32_t addr) const;
      void FlagWrite(uint32_t addr, uint8_t value);

      ZeroMemory memory_;
      MappedMemory mappedMemory_;
      MemoryInterface& biosMemory_;
      ArrayMemory controlMemory_;

      bool biosDoneFlag_;
  };
}

#endif // _EMULATOR_H
