#include <iostream>
#include <unistd.h>

#include "osciemu/osciemu.h"

#include "osciemu-cli/config.h"
#include "osciemu-cli/fileutils.h"
#include "osciemu-cli/runmode.h"
#include "osciemu-cli/stepmode.h"

void printHelp() {
  std::cout << "HELP";
}

int main(int argc, char *argv[]) {
  int ch;

  bool stepModeFlag;
  std::string biosFilename, imageFilename;

  while ((ch = getopt(argc, argv, "i:b:vs")) != -1) {
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
      case 's':
        stepModeFlag = true;
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

  if (stepModeFlag) {
    stepMode(emu);
  } else {
    runMode(emu);
  }
}
