// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* responder.hpp 

*/
#include <indiemotion/common.hpp>
#include <indiemotion/session.hpp>

namespace indiemotion
{
    using ConnectionStartCallback = std::function<void(std::shared_ptr<SessionController>)>;
}
