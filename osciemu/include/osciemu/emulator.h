#ifndef _EMULATOR_H
#define _EMULATOR_H

#include "osciemu/memory.h"
#include "osciemu/instruction.h"

namespace osciemu {
  /**
   * `Emulator` is a class wiring up memory and instruction interpreter
   * to behave like an osci CPU.
   */
  class Emulator : public MemoryInterface {
    public:
      /**
       * Constructor.
       */
      Emulator(MemoryInterface& main, MemoryInterface& bios);
      ~Emulator();
      void Step();

      uint32_t GetSize() const;
      uint8_t GetCell(uint32_t addr) const;
      void SetCell(uint32_t addr, uint8_t value);

      void SetBiosDoneFlag(bool state);

      static const uint32_t kBiosBoundary = 1<<31;
      static const uint32_t kMaxAddress = 0xFFFFFFFF;
      static const uint8_t kNumRegisters = 4;
      static const uint8_t kNumIvts = 1;
      static const uint8_t kNumFlags = 1;
      static const uint32_t kRegisterBoundary = kMaxAddress - kNumRegisters*Instruction::Size;
      static const uint32_t kIvtBoundary = kMaxAddress - (kNumRegisters + kNumIvts)*Instruction::Size;
      static const uint32_t kFlagBoundary = kIvtBoundary - (kNumFlags + 8 - 1)/8; // = kIvtBoundary - ceil(kNumFlags / 8);
      enum FlagEntry {
        kFlagBiosDone
      };
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
