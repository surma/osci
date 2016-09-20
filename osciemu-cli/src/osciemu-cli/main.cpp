#include <iostream>
#include <unistd.h>

#include "osciemu/osciemu.h"

#include "osciemu-cli/config.h"
#include "osciemu-cli/fileutils.h"

void printHelp() {
  std::cout << "HELP";
}

int main(int argc, char *argv[]) {
  int ch;

  std::string biosFilename, imageFilename;

  while ((ch = getopt(argc, argv, "i:b:v")) != -1) {
    switch (ch) {
      case 'v':
        printHelp();
        return 0;
      case 'i':
        imageFilename.assign(optarg);
        break;
      case 'b':
        biosFilename.assign(optarg);
        break;
    }
  }

  if (biosFilename.length() == 0) {
    std::cerr << "No BIOS image specified" << std::endl;
    printHelp();
    return 1;
  }

  if (imageFilename.length() == 0) {
    std::cerr << "No memory image specified" << std::endl;
    printHelp();
    return 1;
  }

  auto image = LoadFileAsArrayMemory(imageFilename);
  auto bios = LoadFileAsArrayMemory(biosFilename);
  auto emu = osciemu::Emulator(image, bios);

  uint32_t i = 0, regAddr, stepCounter = 0;
  while(!emu.IsHalted()) {
    printf("\e[u");
    printf("\e[s");
    printf("(%8d) ip: %08x, ", stepCounter, emu.ip_);
    for(i = 0; i < osciemu::Emulator::kNumRegisters; i++) {
      regAddr = osciemu::Emulator::kRegisterBoundary + i*osciemu::Instruction::Word;
      printf("r%d: %08x, ", i, osciemu::ReadIntFromMemory(emu, regAddr));
    }
    emu.Step();
    stepCounter++;
  }
  printf("\n");
}
