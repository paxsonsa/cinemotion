get_filename_component(TARGET_NAME ${CMAKE_CURRENT_SOURCE_DIR} NAME)
# Always list the source files explicitly, including headers so that they are listed in the IDE
# If you need to use files based on a variable value, use target_sources
set(SRC
    indiemotion-cli.cpp
)

add_executable(${TARGET_NAME} ${SRC})
target_link_libraries(${TARGET_NAME}
#PUBLIC # Useful for libraries, see https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html for more details about transitive usage requirements.
    #libraries/targets to link when linking this library
    #this will automatically setup the needed flags and dependencies when linking against this target
PRIVATE # The following libraries are only linked for this target, and its flags/dependencies will not be used when linking against this target
    general fmt::fmt 
    spdlog::spdlog
    replxx::replxx
    # Boost::program_options

    #debug DEBUGLIBS
    #optimized RELEASELIBS
)
