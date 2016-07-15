#ifndef _MEMORY_H
#define _MEMORY_H

#include <cstdint>
#include <map>

#include "osciemu/config.h"
#include "osciemu/memory.h"

namespace osciemu {
  class MemoryInterface {
    public:
      virtual ~MemoryInterface() {}

      virtual uint32_t GetSize() const = 0;
      virtual uint8_t GetCell(uint32_t addr) const = 0 ;
      virtual void SetCell(uint32_t addr, uint8_t value) = 0;
  };

  class ArrayMemory : public MemoryInterface {
    public:
      ArrayMemory(uint32_t size = 0);
      ~ArrayMemory();

      uint32_t GetSize() const;
      uint8_t GetCell(uint32_t addr) const;
      void SetCell(uint32_t addr, uint8_t value);

    private:
      uint8_t *memory_;
      uint32_t size_;
  };

  typedef std::pair<uint32_t, MemoryInterface&> MappedMemoryEntry;
  class MappedMemory : public MemoryInterface {
    public:
      MappedMemory();
      ~MappedMemory();

      uint32_t GetSize() const;
      uint8_t GetCell(uint32_t addr) const;
      void SetCell(uint32_t addr, uint8_t value);

      void Map(uint32_t start_addr, MemoryInterface& m);
      void Unmap(uint32_t start_addr);
      bool IsMapped(uint32_t addr) const;
    private:
      std::map<uint32_t, MemoryInterface&> maps_;
      uint32_t size_;
      uint32_t RecalculateSize() const;
      MappedMemoryEntry MemoryForAddress(uint32_t addr) const;
  };
}

#endif // _MEMORY_H
