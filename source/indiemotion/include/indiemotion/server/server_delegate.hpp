// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* responder.hpp 

*/
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::server
{
    class ServerDelegate
    {
    private:
    public:
        virtual ~ServerDelegate() {}
        virtual void on_new_session(std::shared_ptr<session::Session>) = 0;
    };

}
