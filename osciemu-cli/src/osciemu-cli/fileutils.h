#ifndef _FILEUTILS_H
#define _FILEUTILS_H

#include <string>
#include "osciemu/osciemu.h"

osciemu::ArrayMemory LoadFileAsArrayMemory(std::string fname);

#endif // _FILEUTILS_H
