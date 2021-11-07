# Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
# Licensed under the MIT License. See LICENSE file in the project root for full license information.

add_subdirectory(indiemotion-protobufs)
add_subdirectory(indiemotion)

if(${${PROJECT_NAME}_BUILD_CLI})
    message("Enabled CLI")
    # add_subdirectory(indiemotion-cli)
#    add_subdirectory(idm)
endif()
