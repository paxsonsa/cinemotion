// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.cpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/server/options.hpp>
#include <indiemotion/server/server_delegate.hpp>

namespace indiemotion::server
{
    class Server
    {
    private:
        std::unique_ptr<ServerDelegate> _m_delegate = nullptr;
        std::unique_ptr<Options> _m_options = nullptr;

    public:
        // Default Constructor
        Server(std::unique_ptr<Options> options, std::unique_ptr<ServerDelegate> delegate)
        {
            std::swap(_m_delegate, delegate);
            std::swap(_m_options, options);
        };

        // Copy the resource (copy constructor)
        Server(const Server &rhs) {}

        // Transfer Ownership (move constructor)
        Server(Server &&rhs) noexcept
        {
            // member = std::exchange(rhs.member, replacevalue);
        }

        // Make type `std::swap`able
        friend void swap(Server &a, Server &b) noexcept
        {
            a.swap(b);
        }

        // Destructor
        ~Server() {}

        // Assignment by Value
        Server &operator=(Server copy)
        {
            copy.swap(*this);
            return *this;
        }

        void swap(Server &rhs) noexcept
        {
            // using std::swap;
            //swap(member, rhs.member);
        }

        void start()
        {
            auto session = std::make_shared<motion::Session>();
            try
            {
                _m_delegate->on_new_session(session);
            }
            catch (const std::exception &e)
            {
                std::cerr << e.what() << '\n';
            }
        }
    };

}
