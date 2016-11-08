cmake_minimum_required (VERSION 3.5)
project (osciemu-cli)

set (OSCIEMU_VERSION_MAJOR 0)
set (OSCIEMU_VERSION_MINOR 0)
set (OSCIEMU_VERSION_PATCH 2)

option(test "Build tests" NO)

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

set (SRCS
  ${PROJECT_SOURCE_DIR}/src/osciemu-cli/fileutils.cpp
  ${PROJECT_SOURCE_DIR}/src/osciemu-cli/runmode.cpp
  ${PROJECT_SOURCE_DIR}/src/osciemu-cli/stepmode.cpp
  ${PROJECT_SOURCE_DIR}/src/osciemu-cli/utils.cpp
)

add_executable (${PROJECT_NAME} ${SRCS} ${PROJECT_SOURCE_DIR}/src/osciemu-cli/main.cpp)
target_link_libraries (${PROJECT_NAME} osciemu)

set (CXX_FEATURES
  cxx_auto_type
)

if (test)
  SET(CMAKE_BUILD_TYPE Debug)
  #add_subdirectory("../vendor/googletest-release-1.7.0" "${CMAKE_BINARY_DIR}/googletest")
  enable_testing()
  include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR})

  set (TEST_SRCS
    "${PROJECT_SOURCE_DIR}/src/osciemu-cli/test/main_test.cpp"
    "${PROJECT_SOURCE_DIR}/src/osciemu-cli/test/fileutils_test.cpp"
  )

  add_executable(cliunittests ${TEST_SRCS} ${SRCS})
  target_compile_features(cliunittests PUBLIC ${CXX_FEATURES})

  target_link_libraries(cliunittests gtest gtest_main osciemu)
  add_test(cliunittests unittests)
endif()