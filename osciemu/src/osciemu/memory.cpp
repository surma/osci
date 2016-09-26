#include <cstdint>
#include <stdexcept>
#include <memory>
#include "osciemu/memory.h"

namespace osciemu {
  ArrayMemory::ArrayMemory(uint32_t size)
    : memory_(new uint8_t[size]), size_(size) {
  }

  ArrayMemory::~ArrayMemory() {
  }

  uint32_t ArrayMemory::GetSize() const {
    return size_;
  }

  uint8_t ArrayMemory::GetCell(uint32_t addr) const {
    if(addr >= size_) {
      throw std::out_of_range("Address out of range");
    }
    return memory_.get()[addr];
  }

  void ArrayMemory::SetCell(uint32_t addr, uint8_t value) {
    if(addr >= size_) {
      throw std::out_of_range("Address out of range");
    }
    memory_.get()[addr] = value;
  }

  MappedMemory::MappedMemory()
   : maps_(), size_(0) {
  }

  MappedMemory::~MappedMemory() {
  }

  uint32_t MappedMemory::GetSize() const {
    return size_;
  }

  uint8_t MappedMemory::GetCell(uint32_t addr) const {
    auto entry = MemoryForAddress(addr);
    return entry.second.GetCell(addr - entry.first);
  }

  void MappedMemory::SetCell(uint32_t addr, uint8_t value) {
    auto entry = MemoryForAddress(addr);
    entry.second.SetCell(addr - entry.first, value);
  }

  void MappedMemory::Map(uint32_t start_addr, MemoryInterface& m) {
    uint32_t end_addr = start_addr + m.GetSize();
    for (auto map_entry : maps_) {
      uint32_t local_start_addr = map_entry.first;
      uint32_t local_end_addr = local_start_addr + map_entry.second.GetSize();
      if (
        (start_addr > local_start_addr && start_addr < local_end_addr)
        ||
        (end_addr > local_start_addr && end_addr < local_end_addr)
      ) {
        throw std::range_error("Mapping overlaps with another mapping");
      }
    }
    maps_.insert(Entry(start_addr, m));
    size_ = RecalculateSize();
  }

  void MappedMemory::Unmap(uint32_t start_addr) {
    auto it = maps_.find(start_addr);
    if (it == maps_.end()) {
      throw std::range_error("No mapping starts at start_addr");
    }
    maps_.erase(it);
    size_ = RecalculateSize();
  }

  bool MappedMemory::IsMapped(uint32_t addr) const {
    try {
      MemoryForAddress(addr);
    }
    catch(const std::out_of_range& e) {
      return false;
    }
    return true;
  }

  uint32_t MappedMemory::RecalculateSize() const {
    uint32_t global_max_addr = 0, local_max_addr = 0;
    for (auto map_entry : maps_) {
      local_max_addr = map_entry.first + map_entry.second.GetSize();
      if (local_max_addr > global_max_addr) {
        global_max_addr = local_max_addr;
      }
    }
    return global_max_addr;
  }

  MappedMemory::Entry MappedMemory::MemoryForAddress(uint32_t addr) const {
    uint32_t map_start_addr, map_end_addr;
    for (auto map_entry : maps_) {
      map_start_addr = map_entry.first;
      map_end_addr = map_entry.first + map_entry.second.GetSize();
      if (map_start_addr <= addr && addr <   map_end_addr) {
        return MappedMemory::Entry(map_entry);
      }
    }
    throw std::out_of_range("No mapping at addr");
  }

  ZeroMemory::ZeroMemory(MemoryInterface& m)
    : memory_(m) {
  }

  ZeroMemory::~ZeroMemory() {
  }

  uint32_t ZeroMemory::GetSize() const {
    return memory_.GetSize();
  }

  uint8_t ZeroMemory::GetCell(uint32_t addr) const {
    try {
      return memory_.GetCell(addr);
    } catch(std::out_of_range e) {
      return 0;
    }
  }

  void ZeroMemory::SetCell(uint32_t addr, uint8_t value) {
    try {
      memory_.SetCell(addr, value);
    } catch(std::out_of_range e) {
    }
  }

  void WriteIntToMemory(MemoryInterface& m, uint32_t addr, int32_t value) {
    m.SetCell(addr+0, (value>> 0)&0xFF);
    m.SetCell(addr+1, (value>> 8)&0xFF);
    m.SetCell(addr+2, (value>>16)&0xFF);
    m.SetCell(addr+3, (value>>24)&0xFF);
  }

  int32_t ReadIntFromMemory(MemoryInterface& m, uint32_t addr) {
    int32_t x = 0;
    x = (x<<8) | m.GetCell(addr+3);
    x = (x<<8) | m.GetCell(addr+2);
    x = (x<<8) | m.GetCell(addr+1);
    x = (x<<8) | m.GetCell(addr+0);
    return x;
  }
}