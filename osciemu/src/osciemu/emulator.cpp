#include <stdexcept>
#include "osciemu/emulator.h"
#include "osciemu/instruction.h"
#include "osciemu/memory.h"


namespace osciemu {
  Emulator::Emulator(MemoryInterface& main, MemoryInterface& bios)
    : ip_(kBiosBoundary),
      mappedMemory_(),
      biosMemory_(bios),
      biosDoneFlag_(false),
      controlMemory_(kMaxAddress - kFlagBoundary),
      memory_(mappedMemory_)
    {
      mappedMemory_.Map(0, main);
      mappedMemory_.Map(kBiosBoundary, bios);
      mappedMemory_.Map(kFlagBoundary, controlMemory_);
    }

  Emulator::~Emulator() {
  }

  void Emulator::Step() {
    auto inst = Instruction::ReadFromMemory(*this, ip_);
    inst.Execute(*this, ip_);
  }

  uint32_t Emulator::GetSize() const {
    return kMaxAddress; // FIXME: Technically, not correct.
  }

  uint8_t Emulator::GetCell(uint32_t addr) const {
    if(addr >= kFlagBoundary && addr < kIvtBoundary) {
      return FlagRead(addr);
    } else {
      return memory_.GetCell(addr);
    }
  }

  void Emulator::SetCell(uint32_t addr, uint8_t value) {
    if(addr >= kFlagBoundary && addr < kIvtBoundary) {
      FlagWrite(addr, value);
    } else {
      memory_.SetCell(addr, value);
    }
  }

  uint8_t Emulator::FlagRead(uint32_t addr) const {
    switch(addr - kFlagBoundary) {
      case 0:
        return biosDoneFlag_; // lol
      default:
        throw std::range_error("Invalid flag read");
    }
  }

  void Emulator::FlagWrite(uint32_t addr, uint8_t value) {
    switch(addr - kFlagBoundary) {
      case 0:
        return SetBiosDoneFlag((value & 1) == 1);
      default:
        throw std::range_error("Invalid flag write");
    }
  }

  void Emulator::SetBiosDoneFlag(bool state) {
    if(state == biosDoneFlag_) {
      return;
    }

    if(state) {
      mappedMemory_.Unmap(kBiosBoundary);
    } else {
      mappedMemory_.Map(kBiosBoundary, biosMemory_);
    }
    biosDoneFlag_ = state;
  }
}