#include <iostream>

#include "utils.h"
#include "osciemu/osciemu.h"

void stepMode(osciemu::Emulator emu) {
  uint32_t i = 0, regAddr, stepCounter = 0;
  std::string lastCommand;
  while (lastCommand != "exit") {
    printf("> ");
    std::cin >> lastCommand;

    if (lastCommand == "exit")
      continue;
    else if (lastCommand == "step") {
      emu.Step();
      printEmulatorState(emu);
      printf("\n");
    } else {
      printf("Unknown command \"%s\"", lastCommand.c_str());
    }
  }
}