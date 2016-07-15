#ifndef _MEMORY_H
#define _MEMORY_H

#include <cstdint>

#include "osciemu/config.h"
#include "osciemu/memory.h"

namespace osciemu {
  class MemoryInterface {
    public:
      virtual ~MemoryInterface() {}

      virtual uint8_t GetCell(uint32_t addr) = 0;
      virtual void SetCell(uint32_t addr, uint8_t value) = 0;
      virtual uint32_t GetSize() = 0;
  };

  class ArrayMemory : MemoryInterface {
    public:
      ArrayMemory(uint32_t size = 0);
      ~ArrayMemory();
      uint32_t GetSize();
      uint8_t GetCell(uint32_t addr);
      void SetCell(uint32_t addr, uint8_t value);

    private:
      uint8_t *memory_;
      uint32_t size_;
  };
}

#endif // _MEMORY_H