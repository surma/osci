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

TEST(InstructionTest, CanExecuteInstruction) {
  auto m = osciemu::ArrayMemory(32);
  uint32_t ip = 0;

  osciemu::Instruction(16, 20, 24, 0).WriteToMemory(m, 0);
  osciemu::WriteIntToMemory(m, 16, 0x31323334);
  osciemu::WriteIntToMemory(m, 20, 0x01020304);

  osciemu::Instruction::Execute(m, ip);

  ASSERT_EQ(ip, 1*osciemu::Instruction::Size);
  ASSERT_EQ(osciemu::ReadIntFromMemory(m, 24), 0x30303030);
}

TEST(InstructionTest, CanExecuteMultipleInstructions) {
  auto m = osciemu::ArrayMemory(512);
  uint32_t ip = 0;

  osciemu::WriteIntToMemory(m, 116, 1);
  osciemu::WriteIntToMemory(m, 120, 2);
  osciemu::WriteIntToMemory(m, 124, 3);

  osciemu::Instruction(116, 120, 128, 12*osciemu::Instruction::Size)
    .WriteToMemory(m,  0*osciemu::Instruction::Size);
  osciemu::Instruction(124, 128, 128,  0)
    .WriteToMemory(m, 12*osciemu::Instruction::Size);
  osciemu::Instruction(128, 116, 128,  0)
    .WriteToMemory(m, 13*osciemu::Instruction::Size);

  osciemu::Instruction::Execute(m, ip);
  ASSERT_EQ(ip, 12*osciemu::Instruction::Size);
  osciemu::Instruction::Execute(m, ip);
  ASSERT_EQ(ip, 13*osciemu::Instruction::Size);
  osciemu::Instruction::Execute(m, ip);
  ASSERT_EQ(ip, 14*osciemu::Instruction::Size);

  ASSERT_EQ(osciemu::ReadIntFromMemory(m, 128), 3);
}

TEST(InstructionTest, RoundsToTheNearestMultiple) {
  auto m = osciemu::ArrayMemory(32);
  uint32_t ip = 0;

  osciemu::Instruction(0, 0, 0, osciemu::Instruction::Size*5+1).WriteToMemory(m, 0);
  osciemu::Instruction::Execute(m, ip);

  ASSERT_EQ(ip, 6*osciemu::Instruction::Size);
}