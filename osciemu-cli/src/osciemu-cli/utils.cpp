#include <iostream>

#include "osciemu/osciemu.h"

void printEmulatorState(osciemu::Emulator emu) {
  uint32_t i, regAddr;
  printf("ip: %08x, ", emu.ip_);
  for(i = 0; i < osciemu::Emulator::kNumRegisters; i++) {
    regAddr = osciemu::Emulator::kRegisterBoundary + i*osciemu::Instruction::Word;
    printf("r%d: %08x, ", i, osciemu::ReadIntFromMemory(emu, regAddr));
  }
}