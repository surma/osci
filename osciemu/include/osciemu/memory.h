#ifndef _MEMORY_H
#define _MEMORY_H

#include <cstdint>
#include <map>
#include <memory>

namespace osciemu {
  /**
   * `MemoryInterface` is the common interface for a
   * byte-addressable memory implementation.
   */
  class MemoryInterface {
    public:
      virtual ~MemoryInterface() {}

      /**
       * `GetSize` returns the size of the memory.
       */
      virtual uint32_t GetSize() const = 0;
      /**
       * `GetCell` returns the byte at address `addr`.
       * @param addr Address of the byte to read
       * @throws std::out_of_range If memory cell does not exist
       */
      virtual uint8_t GetCell(uint32_t addr) const = 0 ;
      /**
       * `SetCell` sets the byte at address `addr` to `value`.
       * @param addr Address of the byte to write
       * @param value Value to store at address
       * @throws std::out_of_range If memory cell does not exist
       */
      virtual void SetCell(uint32_t addr, uint8_t value) = 0;
  };

  /**
   * `ArrayMemory` is a simple consequtive memory implementation
   * using an array.
   */
  class ArrayMemory : public MemoryInterface {
    public:
      /**
       * Constructor.
       * @param size Number of bytes in the memory
       */
      ArrayMemory(uint32_t size = 0);
      ~ArrayMemory();

      /**
       * @see MemoryInterface::GetSize
       */
      uint32_t GetSize() const;
      /**
       * @see MemoryInterface::GetCell
       * @throws std::out_of_range If `addr` is larger than the `ArrayMemory`’s size
       */
      uint8_t GetCell(uint32_t addr) const;
      /**
       * @see MemoryInterface::SetCell
       * @throws std::out_of_range If `addr` is larger than the `ArrayMemory`’s size
       */
      void SetCell(uint32_t addr, uint8_t value);

    private:
      std::shared_ptr<uint8_t> memory_;
      uint32_t size_;
  };

  /**
   * `MappedMemory` maps multiple `MemoryInterface` implementations into one
   * address space.
   *
   * `MappedMemory` takes care of distributing writes and reads according to
   * the mapped layout. `MappedMemory` does not support overlapping mappings.
   */
  class MappedMemory : public MemoryInterface {
    public:
      MappedMemory();
      ~MappedMemory();

      /**
       * @see MemoryInterface::GetSize
       */
      uint32_t GetSize() const;
      /**
       * @see MemoryInterface::GetCell
       * @throws std::out_of_range If `addr` not inside a mapped area
       */
      uint8_t GetCell(uint32_t addr) const;
      /**
       * @see MemoryInterface::SetCell
       * @throws std::out_of_range If `addr` not inside a mapped area
       */
      void SetCell(uint32_t addr, uint8_t value);

      /**
       * `Map` adds a mapping of a memory to the global address space.
       * @param start_addr Address to map the zero address of `m` to
       * @param m Memory to map to the global address space
       * @throws std::range_error If the new mapping would overlap with an existing mapping.
       */
      void Map(uint32_t start_addr, MemoryInterface& m);
      /**
       * `Unmap` removes a mapping of a memory to the global address space.
       * @param start_addr Address the mapping that is to be removed starts
       * @throws std::range_error If there is no mapping starting at `start_addr`
       */
      void Unmap(uint32_t start_addr);
      /**
       * `IsMapped` returns true if `addr` is within a mapped area in the global memory.
       * @param addr Address to check
       */
      bool IsMapped(uint32_t addr) const;
    private:
      typedef std::pair<uint32_t, MemoryInterface&> Entry;
      std::map<uint32_t, MemoryInterface&> maps_;
      uint32_t size_;
      uint32_t RecalculateSize() const;
      Entry MemoryForAddress(uint32_t addr) const;
  };

  /**
   * `WriteIntToMemory` writes a `uint32_t` to memory at the given address in little-endian.
   * @param m Memory to write to
   * @param addr Address to write `value` at
   * @Param value Value to write
   * @throws std::out_of_range If write is invalid
   */
  void WriteIntToMemory(MemoryInterface& m, uint32_t addr, int32_t value);

  /**
   * `ReadIntFromMemory` reads a `uint32_t` from memory at the given address in little-endian.
   * @param m Memory to read from
   * @param addr Address to read from
   * @returns Value read at address
   * @throws std::out_of_range If read is invalid
   */
  int32_t ReadIntFromMemory(MemoryInterface& m, uint32_t addr);
}

#endif // _MEMORY_H
