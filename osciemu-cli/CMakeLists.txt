cmake_minimum_required (VERSION 3.5)
project (osciemu-cli)

set (OSCIEMU_VERSION_MAJOR 0)
set (OSCIEMU_VERSION_MINOR 0)
set (OSCIEMU_VERSION_PATCH 2)

configure_file (
  "${PROJECT_SOURCE_DIR}/src/osciemu-cli/config.h.in"
  "${PROJECT_BINARY_DIR}/src/osciemu-cli/config.h"
)

include_directories (
  "${PROJECT_SOURCE_DIR}/src"
  "${PROJECT_BINARY_DIR}/src"
  "${CMAKE_SOURCE_DIR}/osciemu/include"
  "${CMAKE_BINARY_DIR}/osciemu/include"
)

add_executable (${PROJECT_NAME} src/osciemu-cli/main.cpp)
target_link_libraries (${PROJECT_NAME} osciemu)

