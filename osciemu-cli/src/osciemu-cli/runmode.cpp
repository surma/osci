#include <iostream>

#include "utils.h"
#include "osciemu/osciemu.h"

void runMode(osciemu::Emulator emu) {
  while(!emu.IsHalted()) {
    printf("\e[u");
    printf("\e[s");
    printEmulatorState(emu);
    emu.Step();
  }
  printf("\n");
}