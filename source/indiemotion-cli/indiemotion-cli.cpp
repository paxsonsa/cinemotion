// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* indiemotion-cli.cpp 

*/
#include <iostream>

#include <boost/program_options.hpp>
#include <replxx.hxx>

namespace progopts = boost::program_options;

/**
 * @brief Command Line Options
 * 
 * 
 */
struct cli_options
{
    // Which port should the server use
    int port;
};

int main(int argc, const char **argv)
{
    progopts::options_description descriptor{"IndieMotion Debugger CLI"};

    auto opt = descriptor.add_options();

    cli_options options;
    auto port_opt = progopts::value<int>(&options.port)->default_value(8080)->required();

    opt = opt("help,h", "Print out this help info");
    opt = opt("port,p", port_opt, "Port to register service on.");

    progopts::variables_map var_map;
    progopts::store(progopts::parse_command_line(argc, argv, descriptor), var_map);

    if (var_map.count("help"))
    {
        std::cout << descriptor << "\n";
        return 1;
    }

    // Notify must come after dealing with help or it could throw an exception
    progopts::notify(var_map);

    replxx::Replxx rx;
    rx.install_window_change_handler();

    // load the history file if it exists
    std::string history_file{"$/.indiemotion_history"};
    rx.history_load(history_file);

    std::cout
        << "Welcome to IndieMotion Debugger\n"
        << "Press 'tab' to view autocompletions\n"
        << "Type '.help' for help\n"
        << "Type '.quit' or '.exit' to exit\n\n"
        << "Starting Server: 0.0.0.0:" << options.port << "\n\n";

    std::string prompt{"\x1b[1;32mindiemotion\x1b[0m> "};

    for (;;)
    {
        // display the prompt and retrieve input from the user
        char const *cinput{nullptr};

        do
        {
            cinput = rx.input(prompt);
        } while ((cinput == nullptr) && (errno == EAGAIN));

        if (cinput == nullptr)
        {
            break;
        }

        // change cinput into a std::string
        // easier to manipulate
        std::string input{cinput};

        if (input.empty())
        {
            // user hit enter on an empty line

            continue;
        }
        else if (input.compare(0, 5, ".quit") == 0 || input.compare(0, 5, ".exit") == 0)
        {
            // exit the repl
            rx.history_add(input);
            break;
        }
        else
        {
            // default action
            // echo the input

            rx.print("input: %s\n", input.c_str());

            rx.history_add(input);
            continue;
        }

        // save the history
        rx.history_sync(history_file);

        std::cout << "\nExiting...\n";
    }
    return 0;
}
