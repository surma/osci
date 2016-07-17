#include <stdexcept>

#include "gtest/gtest.h"
#include "osciemu/osciemu.h"


TEST(InstructionTest, ReadWriteCombinationIsIdempotent) {
  auto m = osciemu::ArrayMemory(16);
  auto i1 = osciemu::Instruction(1, 2, 3, 4);
  i1.WriteToMemory(m, 0);

  auto i2 = osciemu::Instruction::ReadFromMemory(m, 0);
  ASSERT_EQ(i1, i2);
}