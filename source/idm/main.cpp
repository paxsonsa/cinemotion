#include <indiemotion/server.hpp>

#include <fmt/core.h>
#include <memory>

int main()
{
    auto options = std::make_unique<indiemotion::server::Options>();
    auto server = std::make_unique<indiemotion::server::Server>(std::move(options));

    fmt::print("Starting Server\n");
    server->start();
    fmt::print("Server Stopped.");

    return 0;
}