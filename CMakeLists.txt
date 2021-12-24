cmake_minimum_required(VERSION 3.20)
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  if(NOT DEFINED WITH_IN_SOURCE_BUILD)
    message(FATAL_ERROR
      "CMake generation for indiemotion is not allowed within the source directory!"
      "\n Remove \"${CMAKE_SOURCE_DIR}/CMakeCache.txt\" and try again from another folder, e.g.:"
      "\n "
      "\n rm CMakeCache.txt"
      "\n mkdir build"
      "\n cd build"
      "\n cmake .."
      "\n "
    )
  endif()
endif()

project(
    "IndieMotion"
    VERSION 1.0.0
    LANGUAGES C CXX
)

# -----------------------------------------------------------------------------
# Build Options
# -----------------------------------------------------------------------------
set( CMAKE_EXPORT_COMPILE_COMMANDS ON )
# Select C++17 as the standard for C++ projects.
set(CMAKE_CXX_STANDARD 17)
# If C++17 is not available, downgrading to an earlier standard is NOT OK.
set(CMAKE_CXX_STANDARD_REQUIRED ON)
# Do not enable compiler specific language extensions.
set(CMAKE_CXX_EXTENSIONS OFF)

# Share Output Locations
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin CACHE INTERNAL "" FORCE)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib CACHE INTERNAL "" FORCE)

# -----------------------------------------------------------------------------
#  Modules and scripts
# -----------------------------------------------------------------------------
# Standard CMake modules
#include(CTest)                          # Must be called before adding tests but after calling project(). This automatically calls enable_testing() and configures ctest targets when using Make/Ninja
include(CMakeDependentOption)           # This is a really useful scripts that creates options that depends on other options. It can even be used with generator expressions !
include(GNUInstallDirs)                 # This will define the default values for installation directories (all platforms even if named GNU)
include(InstallRequiredSystemLibraries) # Tell CMake that the `install` target needs to install required system libraries (eg: Windows SDK)
include(CMakePackageConfigHelpers)      # Helper to create relocatable packages

# Custom modules and scripts

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake") # Make our cmake scripts available

include(LTO)
include(Warnings)
include(CopyDllsForDebug)

# -----------------------------------------------------------------------------
# Project Options
# -----------------------------------------------------------------------------
cmake_dependent_option(${PROJECT_NAME}_BUILD_TESTS
    "Enable ${PROJECT_NAME} project tests targets" ON # By default we want tests if CTest is enabled
    "BUILD_TESTING" OFF # Stay coherent with CTest variables
)

option(${PROJECT_NAME}_BUILD_CLI "Enable/Disable the debugging CLI tools" ON)

# -----------------------------------------------------------------------------
# Dependencies
# -----------------------------------------------------------------------------
find_package(Boost 1.75.0 REQUIRED program_options)
if(Boost_FOUND)
    include_directories(${Boost_INCLUDE_DIRS})
endif()

include(FindProtobuf)
find_package(Protobuf REQUIRED)

# -----------------------------------------------------------------------------
# Libraries and Executables
# -----------------------------------------------------------------------------
add_subdirectory(source)
if (EXISTS "${CMAKE_SOURCE_DIR}/private")
  add_subdirectory(private)
endif()
add_subdirectory(external EXCLUDE_FROM_ALL)
# add_subdirectory(tests)

# -----------------------------------------------------------------------------
# Testing
# -----------------------------------------------------------------------------
# if((CMAKE_PROJECT_NAME STREQUAL IndieMotion OR INDIEMOTION_BUILD_TESTING) AND BUILD_TESTING)
# Let the user add options to the test runner if needed
  set(TEST_RUNNER_PARAMS "--force-colors=false" CACHE STRING "Options to add to our test runners commands")
  enable_testing()
  add_subdirectory(tests)

# endif()
