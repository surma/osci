#include "gtest/gtest.h"
#include "osciemu/osciemu.h"

const int TEST_MEMORY_SIZE = 512;

class ArrayMemoryTest : public ::testing::Test {
  protected:
    ArrayMemoryTest() : memory(TEST_MEMORY_SIZE) {}
    virtual void SetUp() {}

    virtual void TearDown() {}

    osciemu::ArrayMemory memory;
};

TEST_F(ArrayMemoryTest, ReportsCorrectSize) {
  ASSERT_EQ(TEST_MEMORY_SIZE, memory.GetSize());
}

TEST_F(ArrayMemoryTest, StoresData) {
  memory.SetCell(0, 5);
  ASSERT_EQ(memory.GetCell(0), 5);

  const int max_addr = memory.GetSize() -1;
  memory.SetCell(max_addr, 9);
  ASSERT_EQ(memory.GetCell(max_addr), 9);
}
