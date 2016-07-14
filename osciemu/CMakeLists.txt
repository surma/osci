cmake_minimum_required (VERSION 3.5)
project (osciemu)

set (LIBOSCIEMU_VERSION_MAJOR 0)
set (LIBOSCIEMU_VERSION_MINOR 0)
set (LIBOSCIEMU_VERSION_PATCH 1)

configure_file (
  "${PROJECT_SOURCE_DIR}/include/osciemu/config.h.in"
  "${PROJECT_BINARY_DIR}/include/osciemu/config.h"
)

include_directories("${PROJECT_SOURCE_DIR}/include")
include_directories("${PROJECT_BINARY_DIR}/include")
 
set (SRCS
  ${PROJECT_SOURCE_DIR}/src/osciemu/vm.cpp
)

add_library(${PROJECT_NAME} STATIC ${SRCS})
