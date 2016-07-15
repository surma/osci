#include <cstdint>
#include <stdexcept>
#include "osciemu/memory.h"

namespace osciemu {
  ArrayMemory::ArrayMemory(uint32_t size) {
    memory_ = new uint8_t[size];
    size_ = size;
  }

  ArrayMemory::~ArrayMemory() {
    delete[] memory_;
  }

  uint32_t ArrayMemory::GetSize() const {
    return size_;
  }

  uint8_t ArrayMemory::GetCell(uint32_t addr) const {
    return memory_[addr];
  }

  void ArrayMemory::SetCell(uint32_t addr, uint8_t value) {
    memory_[addr] = value;
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
    if (IsMapped(start_addr)) {
      throw new std::range_error("start_addr is a already mapped address");
    }
    maps_.insert(MappedMemoryEntry(start_addr, m));
    size_ = RecalculateSize();
  }

  void MappedMemory::Unmap(uint32_t start_addr) {
    auto it = maps_.find(start_addr);
    if (it == maps_.end()) {
      throw new std::range_error("No mapping starts at start_addr");
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

  MappedMemoryEntry MappedMemory::MemoryForAddress(uint32_t addr) const {
    uint32_t map_start_addr, map_end_addr;
    for (auto map_entry : maps_) {
      map_start_addr = map_entry.first;
      map_end_addr = map_entry.first + map_entry.second.GetSize();
      if (map_start_addr <= addr && addr <   map_end_addr) {
        return MappedMemoryEntry(map_entry);
      }
    }
    throw std::out_of_range("No mapping at addr");
  }
}
