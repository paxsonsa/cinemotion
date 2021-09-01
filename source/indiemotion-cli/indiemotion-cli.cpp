// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* indiemotion-cli.cpp 

*/
#include <iostream>

#include <replxx.hxx>
#include <spdlog/spdlog.h>

int main(int argc, const char** argv) {
    spdlog::info("Welcome to spdlog!");
    spdlog::error("Some error message with arg: {}", 1);
    std::cout << "Hello, World!\n";
    return 0;
}
