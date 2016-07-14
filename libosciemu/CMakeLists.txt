cmake_minimum_required (VERSION 3.5)
project (libosciemu)

set (LIBOSCIEMU_VERSION_MAJOR 0)
set (LIBOSCIEMU_VERSION_MINOR 0)
set (LIBOSCIEMU_VERSION_PATCH 1)

configure_file (
  "${PROJECT_SOURCE_DIR}/include/osciemu/config.h.in"
  "${PROJECT_BINARY_DIR}/include/osciemu/config.h"
)

include_directories("${PROJECT_SOURCE_DIR}/include")
include_directories("${PROJECT_BINARY_DIR}/include")
 
add_library(osciemu src/osciemu/vm.cpp)
