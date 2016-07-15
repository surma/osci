#include <cstdint>
#include "osciemu/memory.h"

namespace osciemu {
  ArrayMemory::ArrayMemory(uint32_t size) {
    memory_ = new uint8_t[size];
    size_ = size;
  }

  ArrayMemory::~ArrayMemory() {
    delete[] memory_;
  }

  uint32_t ArrayMemory::GetSize() {
    return size_;
  }

  uint8_t ArrayMemory::GetCell(uint32_t addr) {
    return memory_[addr];
  }

  void ArrayMemory::SetCell(uint32_t addr, uint8_t value) {
    memory_[addr] = value;
  }
}

