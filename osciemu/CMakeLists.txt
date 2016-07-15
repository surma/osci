cmake_minimum_required (VERSION 3.5)
project (osciemu)

set (LIBOSCIEMU_VERSION_MAJOR 0)
set (LIBOSCIEMU_VERSION_MINOR 0)
set (LIBOSCIEMU_VERSION_PATCH 1)

option(test "Build tests" NO)

configure_file (
  "${PROJECT_SOURCE_DIR}/include/osciemu/config.h.in"
  "${PROJECT_BINARY_DIR}/include/osciemu/config.h"
)

include_directories("${PROJECT_SOURCE_DIR}/include")
include_directories("${PROJECT_BINARY_DIR}/include")

set (SRCS
  ${PROJECT_SOURCE_DIR}/src/osciemu/memory.cpp
)

add_library(${PROJECT_NAME} STATIC ${SRCS})

if (test)
  add_subdirectory("../vendor/googletest-release-1.7.0" "${CMAKE_BINARY_DIR}/googletest")
  enable_testing()
  include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR})

  set (TEST_SRCS
    "${PROJECT_SOURCE_DIR}/src/osciemu/test/main_test.cpp"
    "${PROJECT_SOURCE_DIR}/src/osciemu/test/memory_test.cpp"
  )

  add_executable(unittests ${TEST_SRCS} ${SRCS})
  target_link_libraries(unittests gtest gtest_main)
  add_test(unittests unittests)
endif()