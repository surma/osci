#include <string>
#include <iostream>
#include "osciemu-cli/config.h"
#include "osciemu/osciemu.h"

int main() {
  auto s = osciemu::GetSomeString();
  std::cout << s;
  return 0;
}
