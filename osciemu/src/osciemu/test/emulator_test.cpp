#include <stdexcept>

#include "gtest/gtest.h"
#include "osciemu/osciemu.h"

const int TEST_MEMORY_SIZE = 512;

/**
 * ArrayMemory
 */

class EmulatorTest : public ::testing::Test {
  protected:
    EmulatorTest()
      : mainMemory(TEST_MEMORY_SIZE), biosMemory(TEST_MEMORY_SIZE), emulator(mainMemory, biosMemory) {}

    osciemu::ArrayMemory mainMemory, biosMemory;
    osciemu::Emulator emulator;
};

TEST_F(EmulatorTest, CanReadFromUnmappedMemory) {
  ASSERT_EQ(emulator.GetCell(TEST_MEMORY_SIZE), 0);
}

TEST_F(EmulatorTest, CanWriteToUnmappedMemory) {
  emulator.SetCell(TEST_MEMORY_SIZE, 1);
  ASSERT_EQ(emulator.GetCell(TEST_MEMORY_SIZE), 0);
}

TEST_F(EmulatorTest, CanUnmapBiosViaApi) {
  biosMemory.SetCell(0, 1);

  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 1);
  emulator.SetBiosDoneFlag(true);
  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 0);
  emulator.SetBiosDoneFlag(false);
  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 1);
}

TEST_F(EmulatorTest, CanUnmapBiosViaMemoryWrite) {
  biosMemory.SetCell(0, 1);

  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 1);
  emulator.SetCell(osciemu::Emulator::kFlagBoundary, 1);
  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 0);
  emulator.SetCell(osciemu::Emulator::kFlagBoundary, 0);
  ASSERT_EQ(emulator.GetCell(osciemu::Emulator::kBiosBoundary), 1);
}

TEST_F(EmulatorTest, ExecutesInstructions) {
  osciemu::Instruction(0, 4, 8, 128).WriteToMemory(biosMemory, 0);
  osciemu::Instruction(4, 0, 8, 128).WriteToMemory(biosMemory, 1*osciemu::Instruction::Size);
  osciemu::WriteIntToMemory(emulator, 0, 128);
  osciemu::WriteIntToMemory(emulator, 4, 12);
  emulator.Step();
  ASSERT_EQ(osciemu::ReadIntFromMemory(emulator, 8), 128-12);
  ASSERT_EQ(emulator.ip_, osciemu::Emulator::kBiosBoundary + 1*osciemu::Instruction::Size);
  emulator.Step();
  ASSERT_EQ(osciemu::ReadIntFromMemory(emulator, 8), 12-128);
  ASSERT_EQ(emulator.ip_, 128);
}
