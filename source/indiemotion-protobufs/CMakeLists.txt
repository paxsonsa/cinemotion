get_filename_component(TARGET_NAME ${CMAKE_CURRENT_SOURCE_DIR} NAME)
# Always list the source files explicitly, including headers so that they are listed in the IDE
# If you need to use files based on a variable value, use target_sources
file(GLOB_RECURSE SOURCE_FILES include/**/*.cc)
file(GLOB_RECURSE HEADER_FILES include/**/*.h)
add_library(${TARGET_NAME} ${SOURCE_FILES} ${HEADER_FILES})
set_target_properties(${TARGET_NAME} PROPERTIES
  FRAMEWORK TRUE
  FRAMEWORK_VERSION A # Version "A" is macOS convention
  MACOSX_FRAMEWORK_IDENTIFIER com.andrewpaxson.IndieMotion
)

target_link_libraries(${TARGET_NAME}
    #PUBLIC # Useful for libraries, see https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html for more details about transitive usage requirements.
        #libraries/targets to link when linking this library
        #this will automatically setup the needed flags and dependencies when linking against this target
    INTERFACE # The following libraries are only linked for this target, and its flags/dependencies will not be used when linking against this target
        ${PROTOBUF_LIBRARY}

        #debug DEBUGLIBS
        #optimized RELEASELIBS
)
SET(INDIEMOTION_PROTOBUF_INCLUDE ${CMAKE_CURRENT_SOURCE_DIR}/include PARENT_SCOPE)