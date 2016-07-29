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

  std::cout << "Loading memory image from " << imageFilename << "..." << std::endl;
  auto image = LoadFileAsArrayMemory(imageFilename);
  std::cout << "Done." << std::endl;
  std::cout << "Loading bios image from " << biosFilename << "..." << std::endl;
  auto bios = LoadFileAsArrayMemory(biosFilename);
  std::cout << "Done." << std::endl;
}
