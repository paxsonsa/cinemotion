set(SRC
    indiemotion-cli.cpp
)

add_executable(indiemotion-cli ${SRC})
target_link_libraries(indiemotion-cli
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
