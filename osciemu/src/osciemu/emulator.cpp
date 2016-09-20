#include <stdexcept>
#include "osciemu/emulator.h"
#include "osciemu/instruction.h"
#include "osciemu/memory.h"


namespace osciemu {
  Emulator::Emulator(MemoryInterface& main, MemoryInterface& bios)
    : ip_(kBiosBoundary),
      mappedMemory_(),
      biosMemory_(bios),
      controlMemory_(kMaxAddress - kFlagBoundary),
      memory_(mappedMemory_)
    {
      mappedMemory_.Map(0, main);
      mappedMemory_.Map(kBiosBoundary, bios);
      mappedMemory_.Map(kControlBoundary, controlMemory_);
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
    return memory_.GetCell(addr);
  }

  void Emulator::SetCell(uint32_t addr, uint8_t value) {
    memory_.SetCell(addr, value);
    if(addr >= kFlagBoundary && addr < kIvtBoundary) {
      ProcessFlagChanges();
    }
  }

  inline uint32_t Emulator::GetFlagByteAddress(uint8_t flag, uint8_t byte) const {
    return kFlagBoundary + flag*Instruction::Word + byte;
  }

  void Emulator::ProcessFlagChanges() {
    const auto flag = GetCell(GetFlagByteAddress(0, 0));
    // *un*map the BIOS when bD is true
    SetBiosMap(((flag >> 1) & 1) == 0);
  }

  bool Emulator::IsHalted() const {
    const auto flag = GetCell(GetFlagByteAddress(0, 0));
    return ((flag >> 0) & 1) == 1;
  }

  void Emulator::SetBiosMap(bool newState) {
    const auto isMapped = mappedMemory_.IsMapped(kBiosBoundary);
    if(newState && !isMapped) {
      mappedMemory_.Map(kBiosBoundary, biosMemory_);
    }
    if(!newState && isMapped) {
      mappedMemory_.Unmap(kBiosBoundary);
    }
  }
}