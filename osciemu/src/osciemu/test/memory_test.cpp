#include <stdexcept>

#include "gtest/gtest.h"
#include "osciemu/osciemu.h"

const int TEST_MEMORY_SIZE = 512;

/**
 * ArrayMemory
 */

class ArrayMemoryTest : public ::testing::Test {
  protected:
    ArrayMemoryTest() : memory(TEST_MEMORY_SIZE) {}

    osciemu::ArrayMemory memory;
};

TEST_F(ArrayMemoryTest, ReportsCorrectSize) {
  ASSERT_EQ(memory.GetSize(), TEST_MEMORY_SIZE);
}

TEST_F(ArrayMemoryTest, StoresData) {
  memory.SetCell(0, 5);
  ASSERT_EQ(memory.GetCell(0), 5);

  const int max_addr = memory.GetSize() -1;
  memory.SetCell(max_addr, 9);
  ASSERT_EQ(memory.GetCell(max_addr), 9);
}

/**
 * MappedMemory
 */

class MappedMemoryTest : public ::testing::Test {
  protected:
    virtual void SetUp() {}
    virtual void TearDown() {}

    osciemu::MappedMemory mapped_memory;
};

TEST_F(MappedMemoryTest, ReportsIfAddressIsMapped) {
  ASSERT_FALSE(mapped_memory.IsMapped(0));
}

TEST_F(MappedMemoryTest, ReportsCorrectSizeWithoutMaps) {
  ASSERT_EQ(mapped_memory.GetSize(), 0);
}

TEST_F(MappedMemoryTest, ReportsCorrectSizeWithMap) {
  osciemu::ArrayMemory m1(128), m2(512);

  mapped_memory.Map(0, m1);
  ASSERT_EQ(mapped_memory.GetSize(), 128);

  mapped_memory.Map(512, m2);
  ASSERT_EQ(mapped_memory.GetSize(), 1024);

  mapped_memory.Unmap(0);
  ASSERT_EQ(mapped_memory.GetSize(), 1024);
}

TEST_F(MappedMemoryTest, DistributesReadsAndWrites) {
  osciemu::ArrayMemory m1(16), m2(16);

  mapped_memory.Map(0, m1);
  mapped_memory.Map(16, m2);

  for(int i = 0; i < 32; i++) {
    mapped_memory.SetCell(i, 128+i);
  }

  for(int i = 0; i < 32; i++) {
    ASSERT_EQ(mapped_memory.GetCell(i), 128+i);
  }
  for(int i = 0; i < 16; i++) {
    ASSERT_EQ(m1.GetCell(i), 128+i);
  }
  for(int i = 0; i < 16; i++) {
    ASSERT_EQ(m2.GetCell(i), 128+16+i);
  }
}

TEST_F(MappedMemoryTest, CanTellIfACellIsMapped) {
  osciemu::ArrayMemory m1(8);

  ASSERT_FALSE(mapped_memory.IsMapped(0));
  ASSERT_FALSE(mapped_memory.IsMapped(8));
  ASSERT_FALSE(mapped_memory.IsMapped(15));
  ASSERT_FALSE(mapped_memory.IsMapped(16));

  mapped_memory.Map(8, m1);

  ASSERT_FALSE(mapped_memory.IsMapped(0));
  ASSERT_TRUE(mapped_memory.IsMapped(8));
  ASSERT_TRUE(mapped_memory.IsMapped(15));
  ASSERT_FALSE(mapped_memory.IsMapped(16));
}

TEST_F(MappedMemoryTest, ThrowsOnInvalidMap_A) {
  osciemu::ArrayMemory m1(16), m2(16);

  try {
    // m1[15] =:= m2[0]
    mapped_memory.Map(0, m1);
    mapped_memory.Map(15, m2);
  }
  catch(std::range_error e) {
    SUCCEED();
    return;
  }
  FAIL();
}

TEST_F(MappedMemoryTest, ThrowsOnInvalidMap_B) {
  osciemu::ArrayMemory m1(16), m2(16);

  try {
    // m1[0] =:= m2[15]
    mapped_memory.Map(15, m1);
    mapped_memory.Map(0, m2);
  }
  catch(std::range_error e) {
    SUCCEED();
    return;
  }
  FAIL();
}

TEST(ReadWriteMemory, CombinationIsIdempotent) {
  osciemu::ArrayMemory m(8);
  uint32_t v1 = 0x55AA9966;

  osciemu::WriteIntToMemory(m, 0, v1);
  auto v2 = osciemu::ReadIntFromMemory(m, 0);
  ASSERT_EQ(v1, v2);
}