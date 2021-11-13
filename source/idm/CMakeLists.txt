# Always list the source files explicitly, including headers so that they are listed in the IDE
# If you need to use files based on a variable value, use target_sources

add_executable(idm main.cpp)
target_link_libraries(idm
    #PUBLIC # Useful for libraries, see https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html for more details about transitive usage requirements.
        #libraries/targets to link when linking this library
        #this will automatically setup the needed flags and dependencies when linking against this target
    PRIVATE # The following libraries are only linked for this target, and its flags/dependencies will not be used when linking against this target
        indiemotion

        general fmt::fmt 
        spdlog::spdlog
        Boost::program_options

        #debug DEBUGLIBS
        #optimized RELEASELIBS
)
target_compile_features(idm  PRIVATE cxx_std_17)
target_set_warnings(idm ENABLE pedantic unused)