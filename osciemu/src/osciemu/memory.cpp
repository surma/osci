#include <string>
#include "osciemu/osciemu.h"

namespace osciemu {
  class MemoryInterface {
    public:
      virtual ~MemoryInterface() {}

      virtual uint8 GetCell(uint32 addr) = 0;
      virtual void SetCell(uint32 addr, uint8 value) = 0;
      virtual uint32 GetSize() = 0;
  }

  class ArrayMemory : MemoryInterface {
    public:
      ArrayMemory(size uint32) {
        memory_ = new[] uint8(size);
        size_ = size;
      }

      uint32 GetSize() {
        return size_;
      }

      uint8 GetCell(uint32 addr) {
        return memory_[addr];
      }

      void SetCell(uint32 addr, uint8 value) {
        memory_[addr] = value;
      }

    private:
      uint8[] memory_;
      uint32 size_;
  }
}

