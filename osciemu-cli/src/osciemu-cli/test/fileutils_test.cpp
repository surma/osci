#include <stdexcept>

#include "gtest/gtest.h"
#include "osciemu-cli/fileutils.h"

TEST(LoadFileAsArrayMemory, ReadsFilesCorrectly) {
  auto mem = LoadFileAsArrayMemory("../osciemu-cli/src/osciemu-cli/test/data/memory.img");
  ASSERT_EQ(mem.GetCell(0), 'A');
  ASSERT_EQ(mem.GetCell(1), 'B');
  ASSERT_EQ(mem.GetCell(2), 'C');
  ASSERT_EQ(mem.GetCell(3), '\n');
}
