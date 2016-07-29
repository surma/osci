#include <string>
#include <fstream>
#include <iostream>
#include "osciemu/osciemu.h"
#include "osciemu-cli/fileutils.h"

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