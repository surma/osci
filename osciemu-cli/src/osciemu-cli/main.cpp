#include <iostream>
#include <fstream>
#include <unistd.h>

#include "osciemu-cli/config.h"
#include "osciemu/osciemu.h"

void printHelp() {
  std::cout << "HELP";
}

osciemu::ArrayMemory LoadFileAsArrayMemory(std::string fname) {
  std::ifstream fs(fname, std::ios::ate | std::ios::binary);
  if (!fs.is_open()) {
    std::cerr << "Could not open file: " << fname << std::endl;
    // FIXME: Proper error handling
    return osciemu::ArrayMemory(0);
  }

  // Allocate memory according to file size.
  osciemu::ArrayMemory mem(fs.tellg());
  fs.seekg(0);

  char c;
  uint32_t ctr = 0;
  fs.read(&c, 1);
  while (!fs.eof()) {
    mem.SetCell(ctr++, c);
    fs.read(&c, 1);
  }
  fs.close();
  return mem;
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
